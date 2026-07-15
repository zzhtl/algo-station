use std::collections::HashSet;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::curriculum::{ExerciseDefinition, ExerciseLimits, StarterTemplate};
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

const MAX_SOURCE_BYTES: usize = 64 * 1024;
const MAX_QUEUED_SUBMISSIONS: i64 = 20;

#[derive(Deserialize)]
pub struct ExerciseListQuery {
    stage_id: Option<String>,
    difficulty: Option<String>,
    status: Option<String>,
    cursor: Option<usize>,
    limit: Option<usize>,
}

#[derive(Serialize)]
pub struct ExerciseListItem {
    slug: String,
    lesson_slug: String,
    stage_id: String,
    problem_id: i64,
    title: String,
    difficulty: String,
    summary: String,
    accepted: bool,
}

#[derive(Serialize)]
pub struct ExerciseListResponse {
    items: Vec<ExerciseListItem>,
    next_cursor: Option<usize>,
    total: usize,
}

pub async fn list_exercises(
    State(state): State<AppState>,
    Query(query): Query<ExerciseListQuery>,
) -> ApiResult<Json<ExerciseListResponse>> {
    let accepted = accepted_slugs(&state).await?;
    let mut items = Vec::new();
    for exercise in &state.catalog.exercises {
        let Some(lesson) = state
            .catalog
            .lessons
            .iter()
            .find(|lesson| lesson.slug == exercise.lesson_slug)
        else {
            continue;
        };
        if query
            .stage_id
            .as_ref()
            .is_some_and(|stage_id| stage_id != &lesson.stage_id)
            || query.difficulty.as_ref().is_some_and(|difficulty| {
                !difficulty.eq_ignore_ascii_case(&exercise.difficulty)
            })
        {
            continue;
        }
        let is_accepted = accepted.contains(&exercise.slug);
        if query
            .status
            .as_ref()
            .is_some_and(|status| (status == "accepted") != is_accepted)
        {
            continue;
        }
        items.push(ExerciseListItem {
            slug: exercise.slug.clone(),
            lesson_slug: exercise.lesson_slug.clone(),
            stage_id: lesson.stage_id.clone(),
            problem_id: exercise.problem_id,
            title: exercise.title.clone(),
            difficulty: exercise.difficulty.clone(),
            summary: exercise.summary.clone(),
            accepted: is_accepted,
        });
    }
    let total = items.len();
    let start = query.cursor.unwrap_or(0).min(total);
    let limit = query.limit.unwrap_or(30).clamp(1, 100);
    let end = (start + limit).min(total);
    let next_cursor = (end < total).then_some(end);
    Ok(Json(ExerciseListResponse {
        items: items.drain(start..end).collect(),
        next_cursor,
        total,
    }))
}

#[derive(Serialize)]
pub struct PublicCase {
    name: String,
    input: String,
    expected: String,
}

#[derive(Serialize)]
pub struct ExerciseDetailResponse {
    slug: String,
    problem_id: i64,
    lesson_slug: String,
    title: String,
    difficulty: String,
    summary: String,
    starters: Vec<StarterTemplate>,
    public_cases: Vec<PublicCase>,
    hidden_case_count: usize,
    limits: ExerciseLimits,
    accepted: bool,
}

pub async fn exercise_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> ApiResult<Json<ExerciseDetailResponse>> {
    let exercise = find_exercise(&state, &slug)?;
    let accepted = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM submissions WHERE exercise_slug = ? AND status = 'accepted'",
    )
    .bind(&slug)
    .fetch_one(&state.pool)
    .await?
        > 0;
    Ok(Json(ExerciseDetailResponse {
        slug: exercise.slug.clone(),
        problem_id: exercise.problem_id,
        lesson_slug: exercise.lesson_slug.clone(),
        title: exercise.title.clone(),
        difficulty: exercise.difficulty.clone(),
        summary: exercise.summary.clone(),
        starters: exercise.starters.clone(),
        public_cases: exercise
            .cases
            .iter()
            .filter(|case| case.visibility == "public")
            .map(|case| PublicCase {
                name: case.name.clone(),
                input: case.input.clone(),
                expected: case.expected.clone(),
            })
            .collect(),
        hidden_case_count: exercise
            .cases
            .iter()
            .filter(|case| case.visibility == "hidden")
            .count(),
        limits: exercise.limits.clone(),
        accepted,
    }))
}

#[derive(Deserialize)]
pub struct DraftQuery {
    language: String,
    contract: String,
}

#[derive(Deserialize)]
pub struct DraftInput {
    language: String,
    contract: String,
    code: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct DraftResponse {
    exercise_slug: String,
    language: String,
    contract: String,
    code: String,
    updated_at: String,
}

pub async fn get_draft(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<DraftQuery>,
) -> ApiResult<Json<Option<DraftResponse>>> {
    let exercise = find_exercise(&state, &slug)?;
    validate_target(exercise, &query.language, &query.contract)?;
    let draft = sqlx::query_as::<_, DraftResponse>(
        "SELECT exercise_slug, language, contract, code, updated_at
         FROM exercise_drafts
         WHERE exercise_slug = ? AND language = ? AND contract = ?",
    )
    .bind(&slug)
    .bind(&query.language)
    .bind(&query.contract)
    .fetch_optional(&state.pool)
    .await?;
    Ok(Json(draft))
}

pub async fn put_draft(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(input): Json<DraftInput>,
) -> ApiResult<Json<DraftResponse>> {
    let exercise = find_exercise(&state, &slug)?;
    validate_target(exercise, &input.language, &input.contract)?;
    validate_source(&input.code)?;
    sqlx::query(
        "INSERT INTO exercise_drafts
         (exercise_slug, language, contract, code, updated_at)
         VALUES (?, ?, ?, ?, datetime('now'))
         ON CONFLICT(exercise_slug, language, contract) DO UPDATE SET
             code=excluded.code, updated_at=excluded.updated_at",
    )
    .bind(&slug)
    .bind(&input.language)
    .bind(&input.contract)
    .bind(&input.code)
    .execute(&state.pool)
    .await?;
    let draft = sqlx::query_as::<_, DraftResponse>(
        "SELECT exercise_slug, language, contract, code, updated_at
         FROM exercise_drafts
         WHERE exercise_slug = ? AND language = ? AND contract = ?",
    )
    .bind(&slug)
    .bind(&input.language)
    .bind(&input.contract)
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(draft))
}

#[derive(Deserialize)]
pub struct SubmissionInput {
    exercise_slug: String,
    language: String,
    contract: String,
    source_code: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct SubmissionResponse {
    id: i64,
    exercise_slug: String,
    lesson_slug: String,
    language: String,
    contract: String,
    source_code: String,
    status: String,
    attempts: i64,
    result_json: Option<String>,
    queued_at: String,
    started_at: Option<String>,
    finished_at: Option<String>,
    updated_at: String,
}

#[derive(Serialize)]
pub struct SubmissionPublic {
    id: i64,
    exercise_slug: String,
    lesson_slug: String,
    language: String,
    contract: String,
    source_code: String,
    status: String,
    attempts: i64,
    result: Option<Value>,
    queued_at: String,
    started_at: Option<String>,
    finished_at: Option<String>,
    updated_at: String,
}

impl From<SubmissionResponse> for SubmissionPublic {
    fn from(row: SubmissionResponse) -> Self {
        Self {
            id: row.id,
            exercise_slug: row.exercise_slug,
            lesson_slug: row.lesson_slug,
            language: row.language,
            contract: row.contract,
            source_code: row.source_code,
            status: row.status,
            attempts: row.attempts,
            result: row
                .result_json
                .as_deref()
                .and_then(|value| serde_json::from_str(value).ok()),
            queued_at: row.queued_at,
            started_at: row.started_at,
            finished_at: row.finished_at,
            updated_at: row.updated_at,
        }
    }
}

pub async fn create_submission(
    State(state): State<AppState>,
    Json(input): Json<SubmissionInput>,
) -> ApiResult<(StatusCode, Json<SubmissionPublic>)> {
    let exercise = find_exercise(&state, &input.exercise_slug)?;
    validate_target(exercise, &input.language, &input.contract)?;
    validate_source(&input.source_code)?;
    let queue_size: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM submissions WHERE status IN ('queued', 'running')",
    )
    .fetch_one(&state.pool)
    .await?;
    if queue_size >= MAX_QUEUED_SUBMISSIONS {
        return Err(ApiError::TooManyRequests(
            "judge queue is full; wait for an earlier submission to finish".to_owned(),
        ));
    }
    let result = sqlx::query(
        "INSERT INTO submissions
         (exercise_slug, lesson_slug, language, contract, source_code)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&input.exercise_slug)
    .bind(&exercise.lesson_slug)
    .bind(&input.language)
    .bind(&input.contract)
    .bind(&input.source_code)
    .execute(&state.pool)
    .await?;
    let row = submission_by_id(&state, result.last_insert_rowid()).await?;
    Ok((StatusCode::CREATED, Json(row.into())))
}

pub async fn submission_detail(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<Json<SubmissionPublic>> {
    Ok(Json(submission_by_id(&state, id).await?.into()))
}

#[derive(Deserialize)]
pub struct SubmissionListQuery {
    cursor: Option<i64>,
    limit: Option<usize>,
    exercise_slug: Option<String>,
}

#[derive(Serialize)]
pub struct SubmissionListResponse {
    items: Vec<SubmissionPublic>,
    next_cursor: Option<i64>,
}

pub async fn list_submissions(
    State(state): State<AppState>,
    Query(query): Query<SubmissionListQuery>,
) -> ApiResult<Json<SubmissionListResponse>> {
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let rows = sqlx::query_as::<_, SubmissionResponse>(
        "SELECT id, exercise_slug, lesson_slug, language, contract, source_code, status,
                attempts, result_json, queued_at, started_at, finished_at, updated_at
         FROM submissions
         WHERE (? IS NULL OR id < ?)
           AND (? IS NULL OR exercise_slug = ?)
         ORDER BY id DESC LIMIT ?",
    )
    .bind(query.cursor)
    .bind(query.cursor)
    .bind(&query.exercise_slug)
    .bind(&query.exercise_slug)
    .bind((limit + 1) as i64)
    .fetch_all(&state.pool)
    .await?;
    let has_more = rows.len() > limit;
    let mut items: Vec<SubmissionPublic> = rows.into_iter().take(limit).map(Into::into).collect();
    let next_cursor = has_more.then(|| items.last().map(|item| item.id)).flatten();
    if !has_more {
        items.shrink_to_fit();
    }
    Ok(Json(SubmissionListResponse { items, next_cursor }))
}

#[derive(Serialize)]
pub struct JudgeStatusResponse {
    online: bool,
    queue_size: i64,
    running: i64,
    last_heartbeat_at: Option<String>,
}

pub async fn judge_status(
    State(state): State<AppState>,
) -> ApiResult<Json<JudgeStatusResponse>> {
    let queue_size: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM submissions WHERE status = 'queued'")
            .fetch_one(&state.pool)
            .await?;
    let running: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM submissions WHERE status = 'running'")
            .fetch_one(&state.pool)
            .await?;
    let last_heartbeat_at: Option<String> = sqlx::query_scalar(
        "SELECT MAX(last_heartbeat_at) FROM judge_workers WHERE status = 'online'",
    )
    .fetch_one(&state.pool)
    .await?;
    let online: bool = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM judge_workers
         WHERE status = 'online' AND last_heartbeat_at >= datetime('now', '-30 seconds')",
    )
    .fetch_one(&state.pool)
    .await?
        > 0;
    Ok(Json(JudgeStatusResponse {
        online,
        queue_size,
        running,
        last_heartbeat_at,
    }))
}

async fn accepted_slugs(state: &AppState) -> ApiResult<HashSet<String>> {
    Ok(sqlx::query_scalar(
        "SELECT DISTINCT exercise_slug FROM submissions WHERE status = 'accepted'",
    )
    .fetch_all(&state.pool)
    .await?
    .into_iter()
    .collect())
}

async fn submission_by_id(state: &AppState, id: i64) -> ApiResult<SubmissionResponse> {
    Ok(sqlx::query_as::<_, SubmissionResponse>(
        "SELECT id, exercise_slug, lesson_slug, language, contract, source_code, status,
                attempts, result_json, queued_at, started_at, finished_at, updated_at
         FROM submissions WHERE id = ?",
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?)
}

fn find_exercise<'a>(state: &'a AppState, slug: &str) -> ApiResult<&'a ExerciseDefinition> {
    state
        .catalog
        .exercises
        .iter()
        .find(|exercise| exercise.slug == slug)
        .ok_or(ApiError::NotFound)
}

fn validate_target(
    exercise: &ExerciseDefinition,
    language: &str,
    contract: &str,
) -> ApiResult<()> {
    if !matches!(language, "go" | "rust") || !matches!(contract, "function" | "stdio") {
        return Err(ApiError::BadRequest(
            "language/contract must be go|rust and function|stdio".to_owned(),
        ));
    }
    if !exercise.has_template(language, contract) {
        return Err(ApiError::BadRequest(
            "exercise does not support the requested target".to_owned(),
        ));
    }
    Ok(())
}

fn validate_source(source: &str) -> ApiResult<()> {
    if source.trim().is_empty() {
        return Err(ApiError::BadRequest("source_code must not be empty".to_owned()));
    }
    if source.len() > MAX_SOURCE_BYTES {
        return Err(ApiError::PayloadTooLarge(format!(
            "source_code exceeds {MAX_SOURCE_BYTES} bytes"
        )));
    }
    Ok(())
}

