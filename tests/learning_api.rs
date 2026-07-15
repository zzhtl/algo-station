use std::path::PathBuf;
use std::sync::Arc;

use algo_station::curriculum::CurriculumCatalog;
use algo_station::routes;
use algo_station::state::AppState;
use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use axum::Router;
use serde_json::{json, Value};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt;

async fn test_app() -> (Router, AppState) {
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
    let catalog = Arc::new(CurriculumCatalog::bundled().unwrap());
    let state = AppState {
        pool,
        leetcode_repo: PathBuf::new(),
        catalog,
    };
    let app = Router::new()
        .nest("/api", routes::api_router())
        .with_state(state.clone());
    (app, state)
}

async fn json_response(app: &Router, request: Request<Body>) -> (StatusCode, Value) {
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 2 * 1024 * 1024)
        .await
        .unwrap();
    let value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, value)
}

fn json_request(method: &str, uri: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[tokio::test]
async fn curriculum_and_dashboard_expose_guided_learning_data() {
    let (app, _) = test_app().await;
    let (status, curriculum) = json_response(
        &app,
        Request::get("/api/curriculum").body(Body::empty()).unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(curriculum["stages"].as_array().unwrap().len(), 12);
    assert_eq!(curriculum["summary"]["lesson_count"], 100);

    let (status, dashboard) = json_response(
        &app,
        Request::get("/api/dashboard").body(Body::empty()).unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(dashboard["daily_plan"]["target_minutes"], 60);
    assert_eq!(dashboard["next_lesson"]["slug"], "stage-01-lesson-01");
}

#[tokio::test]
async fn server_derives_completion_and_creates_first_review() {
    let (app, state) = test_app().await;
    let lesson = &state.catalog.lessons[0];
    let answers: Vec<usize> = lesson.quiz.iter().map(|question| question.correct_index).collect();

    let (status, attempt) = json_response(
        &app,
        json_request(
            "POST",
            &format!("/api/lessons/{}/quiz-attempts", lesson.slug),
            json!({ "answers": answers }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(attempt["score"], 100);
    assert_eq!(attempt["passed"], true);

    let (status, progress) = json_response(
        &app,
        json_request(
            "PATCH",
            &format!("/api/lessons/{}/progress", lesson.slug),
            json!({ "animation_completed": true, "note": "能解释不变量" }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(progress["status"], "in_progress");

    let core = &lesson.core_exercise_slugs[0];
    sqlx::query(
        "INSERT INTO submissions
         (exercise_slug, lesson_slug, language, contract, source_code, status, attempts)
         VALUES (?, ?, 'rust', 'stdio', 'fn main() {}', 'accepted', 1)",
    )
    .bind(core)
    .bind(&lesson.slug)
    .execute(&state.pool)
    .await
    .unwrap();

    let (status, progress) = json_response(
        &app,
        json_request(
            "PATCH",
            &format!("/api/lessons/{}/progress", lesson.slug),
            json!({ "animation_completed": true }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(progress["status"], "completed");

    let review: (i64, String) = sqlx::query_as(
        "SELECT step, due_at FROM review_schedules WHERE lesson_slug = ?",
    )
    .bind(&lesson.slug)
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert_eq!(review.0, 0);
    assert!(!review.1.is_empty());
}

#[tokio::test]
async fn prerequisites_are_soft_and_reported_in_lesson_detail() {
    let (app, _) = test_app().await;
    let (status, lesson) = json_response(
        &app,
        Request::get("/api/lessons/stage-01-lesson-02")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(lesson["prerequisites_met"], false);
    assert_eq!(lesson["lesson"]["slug"], "stage-01-lesson-02");
}

#[tokio::test]
async fn exercise_detail_never_exposes_hidden_cases() {
    let (app, state) = test_app().await;
    let exercise = state
        .catalog
        .exercises
        .iter()
        .find(|exercise| {
            exercise.cases.len() > 1
                && exercise.cases[0].input != exercise.cases[1].input
                && exercise.cases[1].visibility == "hidden"
        })
        .unwrap();
    let response = app
        .clone()
        .oneshot(
            Request::get(format!("/api/exercises/{}", exercise.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), 2 * 1024 * 1024)
        .await
        .unwrap();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(!body.contains(&exercise.cases[1].input));
    assert!(!body.contains(&exercise.cases[1].expected));
    let detail: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(detail["hidden_case_count"], 1);
    assert_eq!(detail["starters"].as_array().unwrap().len(), 4);
}

#[tokio::test]
async fn valid_submission_is_queued_and_oversized_source_is_rejected() {
    let (app, state) = test_app().await;
    let exercise = &state.catalog.exercises[0];
    let (status, queued) = json_response(
        &app,
        json_request(
            "POST",
            "/api/submissions",
            json!({
                "exercise_slug": exercise.slug,
                "language": "rust",
                "contract": "stdio",
                "source_code": "fn main() {}"
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(queued["status"], "queued");

    let (status, problem) = json_response(
        &app,
        json_request(
            "POST",
            "/api/submissions",
            json!({
                "exercise_slug": exercise.slug,
                "language": "go",
                "contract": "function",
                "source_code": "x".repeat(65_537)
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(problem["status"], 413);
    assert!(problem.get("error").is_some());
}

#[tokio::test]
async fn submission_queue_applies_the_committed_backpressure_limit() {
    let (app, state) = test_app().await;
    let exercise = &state.catalog.exercises[0];
    for _ in 0..20 {
        sqlx::query(
            "INSERT INTO submissions
             (exercise_slug, lesson_slug, language, contract, source_code)
             VALUES (?, ?, 'rust', 'stdio', 'fn main() {}')",
        )
        .bind(&exercise.slug)
        .bind(&exercise.lesson_slug)
        .execute(&state.pool)
        .await
        .unwrap();
    }

    let (status, problem) = json_response(
        &app,
        json_request(
            "POST",
            "/api/submissions",
            json!({
                "exercise_slug": exercise.slug,
                "language": "rust",
                "contract": "stdio",
                "source_code": "fn main() {}"
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(problem["status"], 429);
}

#[tokio::test]
async fn personal_endpoints_are_no_store_and_errors_are_problem_details() {
    let (app, _) = test_app().await;
    let response = app
        .clone()
        .oneshot(Request::get("/api/dashboard").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert!(response.headers().get("x-request-id").is_some());

    let response = app
        .oneshot(
            Request::get("/api/lessons/does-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(response.headers()["content-type"], "application/problem+json");
    assert!(response.headers().get("x-request-id").is_some());
}

#[tokio::test]
async fn progress_export_is_v2_and_import_still_accepts_v1() {
    let (app, _) = test_app().await;
    let (status, exported) = json_response(
        &app,
        Request::get("/api/progress/export")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(exported["schema_version"], 2);
    assert!(exported["lessons"].is_array());
    assert!(exported["exercise_drafts"].is_array());
    assert!(exported["reviews"].is_array());

    let (status, imported) = json_response(
        &app,
        json_request(
            "POST",
            "/api/progress/import",
            json!({ "training": [], "drafts": [], "bookmarks": [] }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(imported["lessons"], 0);
}

#[tokio::test]
async fn invalid_progress_import_does_not_apply_earlier_records() {
    let (app, state) = test_app().await;
    let (status, problem) = json_response(
        &app,
        json_request(
            "POST",
            "/api/progress/import",
            json!({
                "schema_version": 2,
                "training": [{
                    "article_slug": "must-not-be-written",
                    "status": "started"
                }],
                "lessons": [{
                    "lesson_slug": "unknown-lesson",
                    "status": "completed",
                    "animation_completed": true,
                    "quiz_best_score": 100,
                    "completed_at": "2026-07-15 10:00:00"
                }]
            }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(problem["status"], 400);

    let written: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM training_records WHERE article_slug = 'must-not-be-written'",
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert_eq!(written, 0);
}
