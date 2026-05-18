use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ProblemRow {
    pub id: i64,
    pub slug: String,
    pub title_en: String,
    pub title_cn: String,
    pub difficulty: String,
    pub is_premium: i64,
    pub leetcode_url: String,
    pub leetcode_cn_url: String,
    pub acceptance_rate: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct ProblemListItem {
    pub id: i64,
    pub slug: String,
    pub title_en: String,
    pub title_cn: String,
    pub difficulty: String,
    pub is_premium: bool,
    pub leetcode_url: String,
    pub leetcode_cn_url: String,
    pub acceptance_rate: Option<f64>,
    pub tags: Vec<TagBrief>,
    pub has_article: bool,
}

#[derive(Debug, Serialize, sqlx::FromRow, Clone)]
pub struct TagBrief {
    pub slug: String,
    pub name_en: String,
    pub name_cn: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TagRow {
    pub id: i64,
    pub slug: String,
    pub name_en: String,
    pub name_cn: String,
    pub problem_count: i64,
}

#[derive(Debug, Serialize)]
pub struct ArticleListItem {
    pub slug: String,
    pub title: String,
    pub category: String,
    pub summary: String,
    pub problem_ids: Vec<i64>,
    pub order_in_cat: i64,
}

#[derive(Debug, Serialize)]
pub struct ArticleFull {
    pub slug: String,
    pub title: String,
    pub category: String,
    pub summary: String,
    pub problem_ids: Vec<i64>,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub q: Option<String>,
    pub difficulty: Option<String>,
    pub tag: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub has_article: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct Pagination<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Serialize)]
pub struct Stats {
    pub total_problems: i64,
    pub easy: i64,
    pub medium: i64,
    pub hard: i64,
    pub total_tags: i64,
    pub total_articles: i64,
}
