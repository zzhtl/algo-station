use axum::routing::get;
use axum::Router;

use crate::state::AppState;

mod articles;
mod problems;
mod stats;
mod tags;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/stats", get(stats::stats))
        .route("/problems", get(problems::list))
        .route("/problems/:id", get(problems::detail))
        .route("/problems/:id/statement", get(problems::statement))
        .route("/tags", get(tags::list))
        .route("/articles", get(articles::list))
        .route("/articles/:slug", get(articles::detail))
}

async fn health() -> &'static str {
    "ok"
}
