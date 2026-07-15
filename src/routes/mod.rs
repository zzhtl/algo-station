use axum::http::header::CACHE_CONTROL;
use axum::http::HeaderValue;
use axum::routing::{get, patch, post, put};
use axum::Router;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::set_header::SetResponseHeaderLayer;

use crate::state::AppState;

mod articles;
pub mod course;
mod judge;
mod problems;
mod progress;
mod stats;
mod tags;

pub fn api_router() -> Router<AppState> {
    // 静态性强的只读端点：加 Cache-Control（写端点与个人进度数据不缓存）。
    let cached = Router::new()
        .route("/stats", get(stats::stats))
        .route("/tags", get(tags::list))
        .route("/articles", get(articles::list))
        .route("/articles/:slug", get(articles::detail))
        .layer(SetResponseHeaderLayer::overriding(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=300"),
        ));

    let personal = Router::new()
        .route("/health", get(health))
        .route("/dashboard", get(course::dashboard))
        .route("/curriculum", get(course::curriculum))
        .route("/lessons/:slug", get(course::lesson_detail))
        .route(
            "/lessons/:slug/progress",
            patch(course::patch_progress),
        )
        .route(
            "/lessons/:slug/quiz-attempts",
            post(course::quiz_attempt),
        )
        .route(
            "/daily-plan",
            get(course::daily_plan).put(course::put_daily_plan),
        )
        .route("/reviews", get(course::reviews))
        .route(
            "/reviews/:slug/attempts",
            post(course::review_attempt),
        )
        .route("/exercises", get(judge::list_exercises))
        .route("/exercises/:slug", get(judge::exercise_detail))
        .route(
            "/exercises/:slug/draft",
            get(judge::get_draft).put(judge::put_draft),
        )
        .route(
            "/submissions",
            get(judge::list_submissions).post(judge::create_submission),
        )
        .route("/submissions/:id", get(judge::submission_detail))
        .route("/judge/status", get(judge::judge_status))
        .route("/problems", get(problems::list))
        .route("/problems/:id", get(problems::detail))
        .route("/problems/:id/statement", get(problems::statement))
        .route("/problems/:id/solutions", get(problems::solutions))
        .route("/stats/progress", get(stats::progress_stats))
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
        .layer(SetResponseHeaderLayer::overriding(
            CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ));

    personal
        .merge(cached)
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
}

async fn health() -> &'static str {
    "ok"
}
