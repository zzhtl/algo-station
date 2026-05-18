//! 原创题解：编译时把 content/articles/*.md 嵌入 binary。
//! 启动时把 metadata 灌进 articles 表；正文按 slug 现读现返回（仍在 binary 内）。

use anyhow::Result;
use rust_embed::RustEmbed;
use sqlx::SqlitePool;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/content/articles"]
struct ArticleAssets;

#[derive(Debug, Default)]
pub struct Frontmatter {
    pub title: String,
    pub category: String,
    pub summary: String,
    pub problem_ids: Vec<i64>,
    pub order: i64,
}

/// 把 markdown 文本拆成 (frontmatter, body)。
/// frontmatter 格式：第一行 `---`，到下一个 `---` 之间是 YAML，之后是正文。
pub fn split_frontmatter(text: &str) -> (Option<&str>, &str) {
    let trimmed = text.trim_start_matches('\u{feff}');
    if let Some(rest) = trimmed.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---\n") {
            let fm = &rest[..end];
            let body = &rest[end + 5..];
            return (Some(fm), body);
        }
        if let Some(end) = rest.find("\n---\r\n") {
            let fm = &rest[..end];
            let body = &rest[end + 6..];
            return (Some(fm), body);
        }
    }
    (None, trimmed)
}

pub fn parse_frontmatter(yaml: &str) -> Frontmatter {
    let mut fm = Frontmatter::default();
    for raw in yaml.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else { continue };
        let key = key.trim();
        let value = value.trim().trim_matches(|c: char| c == '"' || c == '\'');
        match key {
            "title" => fm.title = value.to_string(),
            "category" => fm.category = value.to_string(),
            "summary" => fm.summary = value.to_string(),
            "order" => fm.order = value.parse().unwrap_or(0),
            "problem_ids" => {
                // 期望格式: [1, 167, 15]
                fm.problem_ids = value
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .filter_map(|s| s.trim().parse::<i64>().ok())
                    .collect();
            }
            _ => {}
        }
    }
    fm
}

/// 读嵌入 md 文件的 UTF-8 文本。
fn read_asset(file_name: &str) -> Option<String> {
    let asset = ArticleAssets::get(file_name)?;
    std::str::from_utf8(asset.data.as_ref()).ok().map(str::to_string)
}

/// 按 slug 读正文（去除 frontmatter）。供 detail 路由使用。
pub fn read_article_body(slug: &str) -> Option<String> {
    let text = read_asset(&format!("{}.md", slug))?;
    let (_, body) = split_frontmatter(&text);
    Some(body.to_string())
}

/// 启动时把嵌入资源里的 metadata 灌进 articles 表。
pub async fn seed(pool: &SqlitePool) -> Result<usize> {
    sqlx::query("DELETE FROM articles").execute(pool).await?;

    let mut count = 0;
    for file_name in ArticleAssets::iter() {
        let file_name = file_name.as_ref();
        if !file_name.ends_with(".md") {
            continue;
        }
        let slug = file_name.trim_end_matches(".md").to_string();
        let text = match read_asset(file_name) {
            Some(t) => t,
            None => {
                tracing::warn!(file = file_name, "skip: non-utf8 article");
                continue;
            }
        };
        let (fm_text, _body) = split_frontmatter(&text);
        let fm = match fm_text {
            Some(t) => parse_frontmatter(t),
            None => {
                tracing::warn!(file = file_name, "skip: no frontmatter");
                continue;
            }
        };
        if fm.title.is_empty() || fm.category.is_empty() {
            tracing::warn!(file = file_name, "skip: missing title or category");
            continue;
        }

        let problem_ids_json =
            serde_json::to_string(&fm.problem_ids).unwrap_or_else(|_| "[]".into());

        sqlx::query(
            "INSERT INTO articles (slug, title, category, summary, problem_ids, order_in_cat)
             VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT(slug) DO UPDATE SET
                title = excluded.title,
                category = excluded.category,
                summary = excluded.summary,
                problem_ids = excluded.problem_ids,
                order_in_cat = excluded.order_in_cat,
                updated_at = datetime('now')",
        )
        .bind(&slug)
        .bind(&fm.title)
        .bind(&fm.category)
        .bind(&fm.summary)
        .bind(&problem_ids_json)
        .bind(fm.order)
        .execute(pool)
        .await?;

        count += 1;
    }
    tracing::info!("seeded {} articles (embedded)", count);
    Ok(count)
}
