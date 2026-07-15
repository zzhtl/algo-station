use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;

use algo_station::{
    articles_seed, curriculum::CurriculumCatalog, db, default_database_url, problem_seed, routes,
    state::AppState, static_assets,
};
use anyhow::Result;
use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

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
        .max_connections(8)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let leetcode_repo: PathBuf = std::env::var("LEETCODE_REPO_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| algo_station::default_leetcode_repo());

    let problem_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM problems")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);
    if problem_count == 0 {
        if leetcode_repo.join("solution").is_dir() {
            tracing::info!(repo = %leetcode_repo.display(), "首启 seed：扫描 doocs/leetcode 建立索引");
            match problem_seed::seed_from_repo(&pool, &leetcode_repo).await {
                Ok(s) => tracing::info!(
                    "✓ seed 完成：{} 题、{} 标签、跳过 {} 无 README、{} 解析失败",
                    s.problems, s.tags, s.skipped_no_readme, s.skipped_parse_failed
                ),
                Err(e) => tracing::warn!(error = ?e, "seed 失败（题库为空，将继续启动）"),
            }
        } else {
            tracing::warn!(
                "题库为空且未找到 {}/solution；请 clone doocs/leetcode 到 data/leetcode 后运行 scripts/cleanup_leetcode.sh，再 cargo run --bin scrape。",
                leetcode_repo.display()
            );
        }
    }

    if let Err(e) = articles_seed::seed(&pool).await {
        tracing::warn!(error = ?e, "article seeding failed (non-fatal)");
    }

    let catalog = CurriculumCatalog::bundled()?;
    catalog.validate()?;

    let state = AppState {
        pool,
        leetcode_repo,
        catalog: Arc::new(catalog),
    };
    let imported = routes::course::migrate_legacy_progress(&state).await?;
    if imported > 0 {
        tracing::info!(imported, "已迁移旧版训练进度到渐进式课程");
    }

    let app = Router::new()
        .nest("/api", routes::api_router())
        .fallback(static_assets::fallback)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8928);

    let host: IpAddr = std::env::var("HOST")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    let addr = SocketAddr::new(host, port);
    tracing::info!("backend listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
