use axum::extract::State;
use axum::Json;
use serde::Serialize;

use crate::error::ApiResult;
use crate::models::Stats;
use crate::state::AppState;

pub async fn stats(State(state): State<AppState>) -> ApiResult<Json<Stats>> {
    let total_problems: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM problems")
        .fetch_one(&state.pool)
        .await?;
    let easy: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM problems WHERE difficulty = 'Easy'")
        .fetch_one(&state.pool)
        .await?;
    let medium: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM problems WHERE difficulty = 'Medium'")
            .fetch_one(&state.pool)
            .await?;
    let hard: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM problems WHERE difficulty = 'Hard'")
        .fetch_one(&state.pool)
        .await?;
    let total_tags: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tags")
        .fetch_one(&state.pool)
        .await?;
    let total_articles: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM articles")
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(Stats {
        total_problems,
        easy,
        medium,
        hard,
        total_tags,
        total_articles,
    }))
}

#[derive(Serialize, sqlx::FromRow)]
pub struct CategoryProgress {
    pub category: String,
    pub total_articles: i64,
    pub learned: i64,
    pub practiced: i64,
    pub reviewed: i64,
}

#[derive(Serialize)]
pub struct ProgressStats {
    pub categories: Vec<CategoryProgress>,
    pub total_articles: i64,
    pub learned: i64,
    pub practiced: i64,
    pub reviewed: i64,
    pub todo: i64,
}

/// 按原创题解 category 汇总训练完成度（join training_records）。
pub async fn progress_stats(State(state): State<AppState>) -> ApiResult<Json<ProgressStats>> {
    let categories = sqlx::query_as::<_, CategoryProgress>(
        "SELECT a.category AS category,
                COUNT(*) AS total_articles,
                COALESCE(SUM(tr.status = 'learned'), 0) AS learned,
                COALESCE(SUM(tr.status = 'practiced'), 0) AS practiced,
                COALESCE(SUM(tr.status = 'reviewed'), 0) AS reviewed
         FROM articles a
         LEFT JOIN training_records tr ON tr.article_slug = a.slug
         GROUP BY a.category
         ORDER BY a.category",
    )
    .fetch_all(&state.pool)
    .await?;

    let total_articles = categories.iter().map(|c| c.total_articles).sum();
    let learned: i64 = categories.iter().map(|c| c.learned).sum();
    let practiced: i64 = categories.iter().map(|c| c.practiced).sum();
    let reviewed: i64 = categories.iter().map(|c| c.reviewed).sum();
    let todo = total_articles - learned - practiced - reviewed;

    Ok(Json(ProgressStats {
        categories,
        total_articles,
        learned,
        practiced,
        reviewed,
        todo,
    }))
}
