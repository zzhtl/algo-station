use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use algo_station::curriculum::CurriculumCatalog;
use algo_station::judge::{
    claim_next_submission, finish_submission, reap_exhausted_submissions,
    renew_submission_lease, DockerJudge, JudgeOutcome, JudgeStatus,
};
use algo_station::routes::course;
use algo_station::state::AppState;
use algo_station::{db, default_database_url, default_leetcode_repo};
use anyhow::Result;
use sqlx::sqlite::SqlitePoolOptions;
use tracing_subscriber::EnvFilter;

const LEASE_SECONDS: i64 = 45;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| default_database_url());
    db::ensure_database_file(&database_url).await?;
    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let catalog = CurriculumCatalog::bundled()?;
    catalog.validate()?;
    let state = AppState {
        pool: pool.clone(),
        leetcode_repo: std::env::var("LEETCODE_REPO_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| default_leetcode_repo()),
        catalog: Arc::new(catalog),
    };
    let worker_id = std::env::var("JUDGE_WORKER_ID")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            format!(
                "{}-{}",
                std::env::var("HOSTNAME").unwrap_or_else(|_| "local".to_owned()),
                std::process::id()
            )
        });
    let judge = DockerJudge {
        docker_bin: std::env::var("DOCKER_BIN").unwrap_or_else(|_| "docker".to_owned()),
        go_image: std::env::var("JUDGE_GO_IMAGE")
            .unwrap_or_else(|_| "algo-station-go-runner:1.0".to_owned()),
        rust_image: std::env::var("JUDGE_RUST_IMAGE")
            .unwrap_or_else(|_| "algo-station-rust-runner:1.0".to_owned()),
    };
    let allow_rootful = std::env::var("JUDGE_ALLOW_ROOTFUL").as_deref() == Ok("1");
    judge.verify(!allow_rootful).await?;

    register_worker(&pool, &worker_id).await?;
    tracing::info!(worker_id, "Judge Worker 已启动");
    let mut last_heartbeat = Instant::now() - Duration::from_secs(30);

    loop {
        if last_heartbeat.elapsed() >= Duration::from_secs(10) {
            heartbeat_worker(&pool, &worker_id, None).await?;
            last_heartbeat = Instant::now();
        }
        reap_exhausted_submissions(&pool).await?;
        if let Some(job) = claim_next_submission(&pool, &worker_id, LEASE_SECONDS).await? {
            heartbeat_worker(&pool, &worker_id, Some(job.id)).await?;
            tracing::info!(submission_id = job.id, exercise = job.exercise_slug, "开始判题");
            let outcome = if let Some(exercise) = state
                .catalog
                .exercises
                .iter()
                .find(|exercise| exercise.slug == job.exercise_slug)
            {
                let (stop_tx, mut stop_rx) = tokio::sync::watch::channel(false);
                let lease_pool = pool.clone();
                let lease_worker = worker_id.clone();
                let submission_id = job.id;
                let lease_task = tokio::spawn(async move {
                    loop {
                        tokio::select! {
                            changed = stop_rx.changed() => {
                                if changed.is_err() || *stop_rx.borrow() {
                                    break;
                                }
                            }
                            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                                if renew_submission_lease(
                                    &lease_pool,
                                    &lease_worker,
                                    submission_id,
                                    LEASE_SECONDS,
                                )
                                .await
                                .ok() != Some(true)
                                {
                                    break;
                                }
                            }
                        }
                    }
                });
                let outcome = judge.judge(&job, exercise).await;
                let _ = stop_tx.send(true);
                let _ = lease_task.await;
                outcome
            } else {
                JudgeOutcome::internal("exercise is missing from the bundled catalog")
            };
            let finished = finish_submission(&pool, &worker_id, job.id, &outcome).await?;
            if finished
                && outcome.status == JudgeStatus::Accepted
                && let Some(lesson) = state
                    .catalog
                    .lessons
                    .iter()
                    .find(|lesson| lesson.slug == job.lesson_slug)
            {
                course::derive_lesson_progress(&state, lesson).await?;
            }
            heartbeat_worker(&pool, &worker_id, None).await?;
            tracing::info!(
                submission_id = job.id,
                status = outcome.status.as_str(),
                "判题完成"
            );
            continue;
        }

        tokio::select! {
            result = tokio::signal::ctrl_c() => {
                result?;
                break;
            }
            _ = tokio::time::sleep(Duration::from_millis(750)) => {}
        }
    }

    sqlx::query(
        "UPDATE judge_workers SET status = 'offline', current_submission_id = NULL,
                last_heartbeat_at = datetime('now') WHERE worker_id = ?",
    )
    .bind(&worker_id)
    .execute(&pool)
    .await?;
    tracing::info!(worker_id, "Judge Worker 已停止");
    Ok(())
}

async fn register_worker(pool: &sqlx::SqlitePool, worker_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO judge_workers
         (worker_id, version, status, current_submission_id, last_heartbeat_at, started_at)
         VALUES (?, ?, 'online', NULL, datetime('now'), datetime('now'))
         ON CONFLICT(worker_id) DO UPDATE SET
             version=excluded.version,
             status='online',
             current_submission_id=NULL,
             last_heartbeat_at=excluded.last_heartbeat_at,
             started_at=excluded.started_at",
    )
    .bind(worker_id)
    .bind(env!("CARGO_PKG_VERSION"))
    .execute(pool)
    .await?;
    Ok(())
}

async fn heartbeat_worker(
    pool: &sqlx::SqlitePool,
    worker_id: &str,
    submission_id: Option<i64>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE judge_workers SET status = 'online', current_submission_id = ?,
                last_heartbeat_at = datetime('now') WHERE worker_id = ?",
    )
    .bind(submission_id)
    .bind(worker_id)
    .execute(pool)
    .await?;
    Ok(())
}
