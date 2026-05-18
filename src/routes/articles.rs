use axum::extract::{Path, State};
use axum::Json;

use crate::articles_seed;
use crate::error::{ApiError, ApiResult};
use crate::models::{ArticleFull, ArticleListItem};
use crate::state::AppState;

#[derive(sqlx::FromRow)]
struct ArticleRow {
    slug: String,
    title: String,
    category: String,
    summary: String,
    problem_ids: String,
    order_in_cat: i64,
}

pub async fn list(State(state): State<AppState>) -> ApiResult<Json<Vec<ArticleListItem>>> {
    let rows = sqlx::query_as::<_, ArticleRow>(
        "SELECT slug, title, category, summary, problem_ids, order_in_cat
         FROM articles
         ORDER BY category, order_in_cat",
    )
    .fetch_all(&state.pool)
    .await?;

    let items = rows
        .into_iter()
        .map(|r| ArticleListItem {
            slug: r.slug,
            title: r.title,
            category: r.category,
            summary: r.summary,
            problem_ids: serde_json::from_str(&r.problem_ids).unwrap_or_default(),
            order_in_cat: r.order_in_cat,
        })
        .collect();

    Ok(Json(items))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> ApiResult<Json<ArticleFull>> {
    if !slug.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err(ApiError::BadRequest("invalid slug".into()));
    }

    let row = sqlx::query_as::<_, ArticleRow>(
        "SELECT slug, title, category, summary, problem_ids, order_in_cat FROM articles WHERE slug = ?"
    )
    .bind(&slug)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    let body = articles_seed::read_article_body(&row.slug).ok_or(ApiError::NotFound)?;

    Ok(Json(ArticleFull {
        slug: row.slug,
        title: row.title,
        category: row.category,
        summary: row.summary,
        problem_ids: serde_json::from_str(&row.problem_ids).unwrap_or_default(),
        content: body,
    }))
}
