//! 进度 / 草稿 / 收藏 的后端持久化端点。
//! 完全离线、单用户、无鉴权；字段命名沿用本站 API 的 snake_case 约定。

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::ApiResult;
use crate::state::AppState;

// -------------------- 训练进度 --------------------

#[derive(sqlx::FromRow)]
struct TrainingRow {
    article_slug: String,
    status: String,
    pattern_note: String,
    completed_problems: String, // JSON 数组文本
    attempt_result: String,
    stuck_note: String,
    review_note: String,
    updated_at: String,
}

#[derive(Serialize)]
pub struct TrainingRecord {
    pub article_slug: String,
    pub status: String,
    pub pattern_note: String,
    pub completed_problems: Vec<i64>,
    pub attempt_result: String,
    pub stuck_note: String,
    pub review_note: String,
    pub updated_at: String,
}

impl From<TrainingRow> for TrainingRecord {
    fn from(r: TrainingRow) -> Self {
        TrainingRecord {
            completed_problems: serde_json::from_str(&r.completed_problems).unwrap_or_default(),
            article_slug: r.article_slug,
            status: r.status,
            pattern_note: r.pattern_note,
            attempt_result: r.attempt_result,
            stuck_note: r.stuck_note,
            review_note: r.review_note,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct TrainingInput {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub pattern_note: String,
    #[serde(default)]
    pub completed_problems: Vec<i64>,
    #[serde(default)]
    pub attempt_result: String,
    #[serde(default)]
    pub stuck_note: String,
    #[serde(default)]
    pub review_note: String,
}

pub async fn get_training(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<TrainingRecord>>> {
    let rows = sqlx::query_as::<_, TrainingRow>(
        "SELECT article_slug, status, pattern_note, completed_problems, attempt_result, stuck_note, review_note, updated_at
         FROM training_records ORDER BY updated_at DESC",
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(rows.into_iter().map(Into::into).collect()))
}

async fn upsert_training(
    pool: &sqlx::SqlitePool,
    slug: &str,
    input: &TrainingInput,
) -> Result<(), sqlx::Error> {
    let completed = serde_json::to_string(&input.completed_problems).unwrap_or_else(|_| "[]".into());
    sqlx::query(
        "INSERT INTO training_records
            (article_slug, status, pattern_note, completed_problems, attempt_result, stuck_note, review_note, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))
         ON CONFLICT(article_slug) DO UPDATE SET
            status=excluded.status,
            pattern_note=excluded.pattern_note,
            completed_problems=excluded.completed_problems,
            attempt_result=excluded.attempt_result,
            stuck_note=excluded.stuck_note,
            review_note=excluded.review_note,
            updated_at=excluded.updated_at",
    )
    .bind(slug)
    .bind(&input.status)
    .bind(&input.pattern_note)
    .bind(&completed)
    .bind(&input.attempt_result)
    .bind(&input.stuck_note)
    .bind(&input.review_note)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn put_training(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(input): Json<TrainingInput>,
) -> ApiResult<Json<TrainingRecord>> {
    upsert_training(&state.pool, &slug, &input).await?;
    let row = sqlx::query_as::<_, TrainingRow>(
        "SELECT article_slug, status, pattern_note, completed_problems, attempt_result, stuck_note, review_note, updated_at
         FROM training_records WHERE article_slug = ?",
    )
    .bind(&slug)
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(row.into()))
}

// -------------------- 练习代码草稿 --------------------

#[derive(Serialize, sqlx::FromRow)]
pub struct Draft {
    pub problem_id: i64,
    pub lang: String,
    pub code: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct DraftInput {
    pub code: String,
}

pub async fn get_drafts(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<Json<Vec<Draft>>> {
    let rows = sqlx::query_as::<_, Draft>(
        "SELECT problem_id, lang, code, updated_at FROM practice_drafts WHERE problem_id = ?",
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(rows))
}

async fn upsert_draft(
    pool: &sqlx::SqlitePool,
    id: i64,
    lang: &str,
    code: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO practice_drafts (problem_id, lang, code, updated_at)
         VALUES (?, ?, ?, datetime('now'))
         ON CONFLICT(problem_id, lang) DO UPDATE SET code=excluded.code, updated_at=excluded.updated_at",
    )
    .bind(id)
    .bind(lang)
    .bind(code)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn put_draft(
    State(state): State<AppState>,
    Path((id, lang)): Path<(i64, String)>,
    Json(input): Json<DraftInput>,
) -> ApiResult<Json<Draft>> {
    upsert_draft(&state.pool, id, &lang, &input.code).await?;
    let row = sqlx::query_as::<_, Draft>(
        "SELECT problem_id, lang, code, updated_at FROM practice_drafts WHERE problem_id = ? AND lang = ?",
    )
    .bind(id)
    .bind(&lang)
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(row))
}

// -------------------- 题目收藏 --------------------

pub async fn list_bookmarks(State(state): State<AppState>) -> ApiResult<Json<Vec<i64>>> {
    let ids: Vec<i64> =
        sqlx::query_scalar("SELECT problem_id FROM bookmarks ORDER BY created_at DESC")
            .fetch_all(&state.pool)
            .await?;
    Ok(Json(ids))
}

pub async fn add_bookmark(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    sqlx::query("INSERT OR IGNORE INTO bookmarks (problem_id) VALUES (?)")
        .bind(id)
        .execute(&state.pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_bookmark(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    sqlx::query("DELETE FROM bookmarks WHERE problem_id = ?")
        .bind(id)
        .execute(&state.pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

// -------------------- 导出 / 导入（跨设备兜底） --------------------

#[derive(Serialize, Deserialize)]
pub struct ProgressExport {
    #[serde(default)]
    pub training: Vec<TrainingExportRecord>,
    #[serde(default)]
    pub drafts: Vec<DraftExport>,
    #[serde(default)]
    pub bookmarks: Vec<i64>,
    #[serde(default)]
    pub exported_at: String,
}

/// 导出/导入用，可反序列化（TrainingRecord 仅 Serialize）。
#[derive(Serialize, Deserialize)]
pub struct TrainingExportRecord {
    pub article_slug: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub pattern_note: String,
    #[serde(default)]
    pub completed_problems: Vec<i64>,
    #[serde(default)]
    pub attempt_result: String,
    #[serde(default)]
    pub stuck_note: String,
    #[serde(default)]
    pub review_note: String,
}

#[derive(Serialize, Deserialize)]
pub struct DraftExport {
    pub problem_id: i64,
    pub lang: String,
    #[serde(default)]
    pub code: String,
}

#[derive(Serialize)]
pub struct ImportResult {
    pub training: usize,
    pub drafts: usize,
    pub bookmarks: usize,
}

pub async fn export_all(State(state): State<AppState>) -> ApiResult<Json<ProgressExport>> {
    let training: Vec<TrainingExportRecord> = sqlx::query_as::<_, TrainingRow>(
        "SELECT article_slug, status, pattern_note, completed_problems, attempt_result, stuck_note, review_note, updated_at
         FROM training_records ORDER BY article_slug",
    )
    .fetch_all(&state.pool)
    .await?
    .into_iter()
    .map(|r| TrainingExportRecord {
        completed_problems: serde_json::from_str(&r.completed_problems).unwrap_or_default(),
        article_slug: r.article_slug,
        status: r.status,
        pattern_note: r.pattern_note,
        attempt_result: r.attempt_result,
        stuck_note: r.stuck_note,
        review_note: r.review_note,
    })
    .collect();

    let drafts: Vec<DraftExport> = sqlx::query_as::<_, Draft>(
        "SELECT problem_id, lang, code, updated_at FROM practice_drafts ORDER BY problem_id, lang",
    )
    .fetch_all(&state.pool)
    .await?
    .into_iter()
    .map(|d| DraftExport {
        problem_id: d.problem_id,
        lang: d.lang,
        code: d.code,
    })
    .collect();

    let bookmarks: Vec<i64> =
        sqlx::query_scalar("SELECT problem_id FROM bookmarks ORDER BY created_at DESC")
            .fetch_all(&state.pool)
            .await?;

    let exported_at: String = sqlx::query_scalar("SELECT datetime('now')")
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(ProgressExport {
        training,
        drafts,
        bookmarks,
        exported_at,
    }))
}

pub async fn import_all(
    State(state): State<AppState>,
    Json(data): Json<ProgressExport>,
) -> ApiResult<Json<ImportResult>> {
    let result = ImportResult {
        training: data.training.len(),
        drafts: data.drafts.len(),
        bookmarks: data.bookmarks.len(),
    };

    for r in &data.training {
        let input = TrainingInput {
            status: r.status.clone(),
            pattern_note: r.pattern_note.clone(),
            completed_problems: r.completed_problems.clone(),
            attempt_result: r.attempt_result.clone(),
            stuck_note: r.stuck_note.clone(),
            review_note: r.review_note.clone(),
        };
        upsert_training(&state.pool, &r.article_slug, &input).await?;
    }
    for d in &data.drafts {
        upsert_draft(&state.pool, d.problem_id, &d.lang, &d.code).await?;
    }
    for b in &data.bookmarks {
        sqlx::query("INSERT OR IGNORE INTO bookmarks (problem_id) VALUES (?)")
            .bind(b)
            .execute(&state.pool)
            .await?;
    }

    Ok(Json(result))
}
