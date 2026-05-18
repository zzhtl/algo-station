use axum::extract::State;
use axum::Json;

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
