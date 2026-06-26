use axum::http::header::CACHE_CONTROL;
use axum::http::HeaderValue;
use axum::routing::{get, post, put};
use axum::Router;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::state::AppState;

mod articles;
mod problems;
mod progress;
mod stats;
mod tags;

pub fn api_router() -> Router<AppState> {
    // 静态性强的只读端点：加 Cache-Control（写端点与个人进度数据不缓存）。
    let cached = Router::new()
        .route("/stats", get(stats::stats))
        .route("/stats/progress", get(stats::progress_stats))
        .route("/tags", get(tags::list))
        .route("/articles", get(articles::list))
        .route("/articles/:slug", get(articles::detail))
        .layer(SetResponseHeaderLayer::overriding(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=300"),
        ));

    Router::new()
        .route("/health", get(health))
        .route("/problems", get(problems::list))
        .route("/problems/:id", get(problems::detail))
        .route("/problems/:id/statement", get(problems::statement))
        .route("/problems/:id/solutions", get(problems::solutions))
        .route("/progress/training", get(progress::get_training))
        .route("/progress/training/:slug", put(progress::put_training))
        .route("/progress/drafts/:id", get(progress::get_drafts))
        .route("/progress/drafts/:id/:lang", put(progress::put_draft))
        .route("/progress/export", get(progress::export_all))
        .route("/progress/import", post(progress::import_all))
        .route("/bookmarks", get(progress::list_bookmarks))
        .route(
            "/bookmarks/:id",
            put(progress::add_bookmark).delete(progress::remove_bookmark),
        )
        .merge(cached)
}

async fn health() -> &'static str {
    "ok"
}
