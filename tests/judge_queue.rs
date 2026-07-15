use std::path::Path;

use algo_station::judge::{
    claim_next_submission, docker_sandbox_args, outputs_match, reap_exhausted_submissions,
};
use sqlx::sqlite::SqlitePoolOptions;

async fn pool() -> sqlx::SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate::Migrator::new(std::path::Path::new("./migrations"))
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();
    pool
}

#[tokio::test]
async fn claiming_is_ordered_and_increments_attempts() {
    let pool = pool().await;
    for slug in ["exercise-1", "exercise-2"] {
        sqlx::query(
            "INSERT INTO submissions
             (exercise_slug, lesson_slug, language, contract, source_code)
             VALUES (?, 'lesson-1', 'rust', 'stdio', 'fn main() {}')",
        )
        .bind(slug)
        .execute(&pool)
        .await
        .unwrap();
    }

    let first = claim_next_submission(&pool, "worker-a", 45)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(first.exercise_slug, "exercise-1");
    assert_eq!(first.attempts, 1);

    let second = claim_next_submission(&pool, "worker-b", 45)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(second.exercise_slug, "exercise-2");

    sqlx::query("UPDATE submissions SET lease_until = datetime('now', '-1 second') WHERE id = ?")
        .bind(first.id)
        .execute(&pool)
        .await
        .unwrap();
    let retried = claim_next_submission(&pool, "worker-c", 45)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(retried.id, first.id);
    assert_eq!(retried.attempts, 2);
}

#[tokio::test]
async fn exhausted_expired_lease_becomes_a_terminal_error_once() {
    let pool = pool().await;
    sqlx::query(
        "INSERT INTO submissions
         (exercise_slug, lesson_slug, language, contract, source_code, status, attempts,
          lease_owner, lease_until)
         VALUES ('exercise-1', 'lesson-1', 'go', 'stdio', 'package main', 'running', 3,
                 'dead-worker', datetime('now', '-1 second'))",
    )
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(reap_exhausted_submissions(&pool).await.unwrap(), 1);
    let row: (String, Option<String>, Option<String>, Option<String>) = sqlx::query_as(
        "SELECT status, result_json, lease_owner, finished_at FROM submissions WHERE id = 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.0, "internal_error");
    assert!(row.1.unwrap().contains("lease expired after 3 attempts"));
    assert!(row.2.is_none());
    assert!(row.3.is_some());
    assert_eq!(reap_exhausted_submissions(&pool).await.unwrap(), 0);
}

#[test]
fn docker_arguments_enforce_the_sandbox_boundary() {
    let args = docker_sandbox_args(Path::new("/tmp/judge-1"), 256);
    let joined = args.join(" ");
    assert!(joined.contains("run --rm --interactive"));
    assert!(joined.contains("--network none"));
    assert!(joined.contains("--read-only"));
    assert!(joined.contains("--cap-drop ALL"));
    assert!(joined.contains("no-new-privileges"));
    assert!(joined.contains("--pids-limit 64"));
    assert!(joined.contains("--memory 256m"));
    assert!(joined.contains("--user 65532:65532"));
    assert!(joined.contains("/tmp:rw,nosuid,size=128m,mode=1777"));
    assert!(!args.iter().any(|argument| argument == "sh" || argument == "bash"));
}

#[test]
fn output_comparison_ignores_line_endings_and_trailing_space_only() {
    assert!(outputs_match("1 2 3  \r\n", "1 2 3\n"));
    assert!(!outputs_match("1  2\n", "1 2\n"));
}
