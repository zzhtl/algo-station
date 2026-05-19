use axum::extract::{Path, State};
use axum::Json;
use std::collections::{HashMap, HashSet};

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

    let parsed: Vec<(ArticleRow, Vec<i64>)> = rows
        .into_iter()
        .map(|r| {
            let problem_ids = serde_json::from_str(&r.problem_ids).unwrap_or_default();
            (r, problem_ids)
        })
        .collect();
    let difficulty_map = load_problem_difficulties(
        &state,
        parsed
            .iter()
            .flat_map(|(_, ids)| ids.iter().copied())
            .collect(),
    )
    .await?;

    let items = parsed
        .into_iter()
        .map(|(r, problem_ids)| ArticleListItem {
            difficulty: infer_article_difficulty(&problem_ids, &difficulty_map),
            slug: r.slug,
            title: r.title,
            category: r.category,
            summary: r.summary,
            problem_ids,
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
    let problem_ids: Vec<i64> = serde_json::from_str(&row.problem_ids).unwrap_or_default();
    let difficulty_map = load_problem_difficulties(&state, problem_ids.iter().copied().collect()).await?;

    Ok(Json(ArticleFull {
        slug: row.slug,
        title: row.title,
        category: row.category,
        difficulty: infer_article_difficulty(&problem_ids, &difficulty_map),
        summary: row.summary,
        problem_ids,
        content: body,
    }))
}

async fn load_problem_difficulties(
    state: &AppState,
    problem_ids: HashSet<i64>,
) -> ApiResult<HashMap<i64, String>> {
    if problem_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let ids: Vec<i64> = problem_ids.into_iter().collect();
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT id, difficulty FROM problems WHERE id IN ({})",
        placeholders
    );
    let mut q = sqlx::query_as::<_, ProblemDifficultyRow>(&sql);
    for id in &ids {
        q = q.bind(id);
    }
    let rows = q.fetch_all(&state.pool).await?;
    Ok(rows
        .into_iter()
        .map(|row| (row.id, row.difficulty))
        .collect())
}

fn infer_article_difficulty(
    problem_ids: &[i64],
    difficulty_map: &HashMap<i64, String>,
) -> String {
    let rank = problem_ids
        .iter()
        .filter_map(|id| difficulty_map.get(id))
        .map(|difficulty| difficulty_rank(difficulty))
        .max()
        .unwrap_or(2);

    match rank {
        1 => "Easy",
        3 => "Hard",
        _ => "Medium",
    }
    .to_string()
}

fn difficulty_rank(difficulty: &str) -> i64 {
    match difficulty {
        "Easy" => 1,
        "Hard" => 3,
        _ => 2,
    }
}

#[derive(sqlx::FromRow)]
struct ProblemDifficultyRow {
    id: i64,
    difficulty: String,
}
