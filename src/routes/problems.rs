use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::{ApiError, ApiResult};
use crate::models::{ListQuery, Pagination, ProblemListItem, ProblemRow, TagBrief};
use crate::problem_seed::{self, Lang};
use crate::state::AppState;

const DEFAULT_PAGE_SIZE: i64 = 30;
const MAX_PAGE_SIZE: i64 = 200;

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> ApiResult<Json<Pagination<ProblemListItem>>> {
    let page = q.page.unwrap_or(1).max(1);
    let page_size = q
        .page_size
        .unwrap_or(DEFAULT_PAGE_SIZE)
        .clamp(1, MAX_PAGE_SIZE);
    let offset = (page - 1) * page_size;

    let mut where_clauses: Vec<String> = Vec::new();
    let mut binds: Vec<String> = Vec::new();

    if let Some(diff) = q.difficulty.as_deref() {
        if !diff.is_empty() {
            where_clauses.push("p.difficulty = ?".into());
            binds.push(normalize_difficulty(diff));
        }
    }

    if let Some(tag) = q.tag.as_deref() {
        if !tag.is_empty() {
            where_clauses.push(
                "p.id IN (SELECT pt.problem_id FROM problem_tags pt JOIN tags t ON t.id = pt.tag_id WHERE t.slug = ?)"
                    .into(),
            );
            binds.push(tag.to_string());
        }
    }

    if let Some(needle) = q.q.as_deref() {
        let needle = needle.trim();
        if !needle.is_empty() {
            if let Ok(num) = needle.parse::<i64>() {
                where_clauses.push("(p.id = ? OR p.title_en LIKE ? OR p.title_cn LIKE ?)".into());
                binds.push(num.to_string());
                binds.push(format!("%{}%", needle));
                binds.push(format!("%{}%", needle));
            } else {
                where_clauses
                    .push("(p.title_en LIKE ? OR p.title_cn LIKE ? OR p.slug LIKE ?)".into());
                binds.push(format!("%{}%", needle));
                binds.push(format!("%{}%", needle));
                binds.push(format!("%{}%", needle));
            }
        }
    }

    if q.has_article == Some(true) {
        where_clauses.push(
            "EXISTS (SELECT 1 FROM articles a, json_each(a.problem_ids) je WHERE CAST(je.value AS INTEGER) = p.id)"
                .into(),
        );
    }

    let where_sql = if where_clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };

    let count_sql = format!("SELECT COUNT(*) FROM problems p {}", where_sql);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for b in &binds {
        count_q = count_q.bind(b);
    }
    let total: i64 = count_q.fetch_one(&state.pool).await?;

    let list_sql = format!(
        "SELECT p.id, p.slug, p.title_en, p.title_cn, p.difficulty, p.is_premium, p.leetcode_url, p.leetcode_cn_url, p.acceptance_rate
         FROM problems p
         {}
         ORDER BY p.id ASC
         LIMIT ? OFFSET ?",
        where_sql
    );
    let mut list_q = sqlx::query_as::<_, ProblemRow>(&list_sql);
    for b in &binds {
        list_q = list_q.bind(b);
    }
    let rows = list_q
        .bind(page_size)
        .bind(offset)
        .fetch_all(&state.pool)
        .await?;

    let problem_ids: Vec<i64> = rows.iter().map(|r| r.id).collect();
    let tag_map = load_tags_for(&state, &problem_ids).await?;
    let article_map = load_article_flags(&state, &problem_ids).await?;

    let items: Vec<ProblemListItem> = rows
        .into_iter()
        .map(|r| ProblemListItem {
            id: r.id,
            slug: r.slug,
            title_en: r.title_en,
            title_cn: r.title_cn,
            difficulty: r.difficulty,
            is_premium: r.is_premium != 0,
            leetcode_url: r.leetcode_url,
            leetcode_cn_url: r.leetcode_cn_url,
            acceptance_rate: r.acceptance_rate,
            tags: tag_map.get(&r.id).cloned().unwrap_or_default(),
            has_article: *article_map.get(&r.id).unwrap_or(&false),
        })
        .collect();

    Ok(Json(Pagination {
        items,
        total,
        page,
        page_size,
    }))
}

#[derive(Serialize)]
pub struct ProblemDetail {
    #[serde(flatten)]
    pub base: ProblemListItem,
    pub related_articles: Vec<RelatedArticle>,
    pub has_statement: bool,
    pub statement_source: Option<String>,
    pub statement_source_url: Option<String>,
    pub statement_license: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct RelatedArticle {
    pub slug: String,
    pub title: String,
    pub category: String,
    pub summary: String,
}

pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<Json<ProblemDetail>> {
    let row = sqlx::query_as::<_, ProblemDetailRow>(
        "SELECT id, slug, title_en, title_cn, difficulty, is_premium, leetcode_url, leetcode_cn_url, acceptance_rate, repo_dir FROM problems WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    let tag_map = load_tags_for(&state, &[id]).await?;
    let tags = tag_map.get(&id).cloned().unwrap_or_default();

    let related: Vec<RelatedArticle> = sqlx::query_as::<_, RelatedArticle>(
        "SELECT a.slug, a.title, a.category, a.summary
         FROM articles a, json_each(a.problem_ids) je
         WHERE CAST(je.value AS INTEGER) = ?
         ORDER BY a.category, a.order_in_cat",
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await?;

    let has_article = !related.is_empty();

    let has_statement = !row.repo_dir.is_empty()
        && state
            .leetcode_repo
            .join(&row.repo_dir)
            .join("README.md")
            .is_file();

    let source_url = if row.repo_dir.is_empty() {
        None
    } else {
        Some(format!(
            "https://github.com/doocs/leetcode/tree/main/{}",
            row.repo_dir
        ))
    };

    Ok(Json(ProblemDetail {
        base: ProblemListItem {
            id: row.id,
            slug: row.slug,
            title_en: row.title_en,
            title_cn: row.title_cn,
            difficulty: row.difficulty,
            is_premium: row.is_premium != 0,
            leetcode_url: row.leetcode_url,
            leetcode_cn_url: row.leetcode_cn_url,
            acceptance_rate: row.acceptance_rate,
            tags,
            has_article,
        },
        related_articles: related,
        has_statement,
        statement_source: has_statement.then(|| "doocs/leetcode".to_string()),
        statement_source_url: if has_statement { source_url } else { None },
        statement_license: has_statement.then(|| "CC-BY-SA-4.0".to_string()),
    }))
}

#[derive(sqlx::FromRow)]
struct ProblemDetailRow {
    id: i64,
    slug: String,
    title_en: String,
    title_cn: String,
    difficulty: String,
    is_premium: i64,
    leetcode_url: String,
    leetcode_cn_url: String,
    acceptance_rate: Option<f64>,
    repo_dir: String,
}

#[derive(Deserialize)]
pub struct StatementQuery {
    /// "cn"（默认）或 "en"
    pub lang: Option<String>,
}

#[derive(Serialize)]
pub struct StatementResponse {
    pub problem_id: i64,
    pub lang: String,
    pub content: String,
    pub source: String,
    pub source_url: String,
    pub license: String,
}

pub async fn statement(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(q): Query<StatementQuery>,
) -> ApiResult<Json<StatementResponse>> {
    let repo_dir: Option<String> =
        sqlx::query_scalar("SELECT repo_dir FROM problems WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.pool)
            .await?;
    let repo_dir = repo_dir.ok_or(ApiError::NotFound)?;
    if repo_dir.is_empty() {
        return Err(ApiError::NotFound);
    }

    let (lang_label, lang_enum) = match q.lang.as_deref() {
        Some("en") | Some("EN") => ("en", Lang::En),
        _ => ("cn", Lang::Cn),
    };

    let content = problem_seed::read_statement(&state.leetcode_repo, &repo_dir, lang_enum)
        .map_err(|_| ApiError::NotFound)?;

    Ok(Json(StatementResponse {
        problem_id: id,
        lang: lang_label.to_string(),
        content,
        source: "doocs/leetcode".into(),
        source_url: format!("https://github.com/doocs/leetcode/tree/main/{}", repo_dir),
        license: "CC-BY-SA-4.0".into(),
    }))
}

fn normalize_difficulty(s: &str) -> String {
    match s.to_ascii_lowercase().as_str() {
        "easy" | "简单" | "1" => "Easy".into(),
        "medium" | "中等" | "2" => "Medium".into(),
        "hard" | "困难" | "3" => "Hard".into(),
        other => other.to_string(),
    }
}

async fn load_tags_for(
    state: &AppState,
    problem_ids: &[i64],
) -> ApiResult<HashMap<i64, Vec<TagBrief>>> {
    if problem_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let placeholders = problem_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT pt.problem_id, t.slug, t.name_en, t.name_cn
         FROM problem_tags pt
         JOIN tags t ON t.id = pt.tag_id
         WHERE pt.problem_id IN ({})
         ORDER BY t.slug",
        placeholders
    );
    let mut q = sqlx::query_as::<_, ProblemTagRow>(&sql);
    for id in problem_ids {
        q = q.bind(id);
    }
    let rows = q.fetch_all(&state.pool).await?;
    let mut map: HashMap<i64, Vec<TagBrief>> = HashMap::new();
    for r in rows {
        map.entry(r.problem_id).or_default().push(TagBrief {
            slug: r.slug,
            name_en: r.name_en,
            name_cn: r.name_cn,
        });
    }
    Ok(map)
}

#[derive(sqlx::FromRow)]
struct ProblemTagRow {
    problem_id: i64,
    slug: String,
    name_en: String,
    name_cn: String,
}

async fn load_article_flags(
    state: &AppState,
    problem_ids: &[i64],
) -> ApiResult<HashMap<i64, bool>> {
    if problem_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let placeholders = problem_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT DISTINCT CAST(je.value AS INTEGER) AS pid
         FROM articles a, json_each(a.problem_ids) je
         WHERE CAST(je.value AS INTEGER) IN ({})",
        placeholders
    );
    let mut q = sqlx::query_scalar::<_, i64>(&sql);
    for id in problem_ids {
        q = q.bind(id);
    }
    let ids = q.fetch_all(&state.pool).await?;
    let mut map = HashMap::new();
    for id in ids {
        map.insert(id, true);
    }
    Ok(map)
}
