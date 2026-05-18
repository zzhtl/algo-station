//! 离线题库索引化命令。
//!
//! 从 `data/leetcode/`（doocs/leetcode 瘦身后只含 README.md / README_EN.md）扫盘，
//! 建立 SQLite 索引（problems / tags / problem_tags / problems_fts）。题面 markdown 不入库。
//!
//! 完全离线，不调任何网络接口。
//!
//! 用法：
//!   cargo run --release --bin scrape -- [--repo data/leetcode]

use std::path::PathBuf;

use anyhow::{Context, Result};
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Debug, Default)]
struct Args {
    repo: Option<PathBuf>,
}

fn parse_args() -> Result<Args> {
    let mut args = Args::default();
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--repo" => args.repo = Some(PathBuf::from(it.next().context("--repo 缺值")?)),
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            _ => anyhow::bail!("未知参数：{a}"),
        }
    }
    Ok(args)
}

fn print_help() {
    eprintln!(
        "scrape - 扫 data/leetcode/ 建立 SQLite 索引（完全离线）\n\n\
         用法：\n  \
         cargo run --release --bin scrape -- [--repo <path>]\n\n\
         参数：\n  \
         --repo <path>   doocs/leetcode 本地路径（默认按 LEETCODE_REPO_DIR / data/leetcode 解析）"
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let args = parse_args()?;

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| algo_station::default_database_url());
    algo_station::db::ensure_database_file(&database_url).await?;

    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let repo = args
        .repo
        .or_else(algo_station::problem_seed::resolve_repo_path)
        .context(
            "找不到 data/leetcode/ 目录。请用 --repo 指定，或先 clone doocs/leetcode 到 data/leetcode 并运行 scripts/cleanup_leetcode.sh",
        )?;

    println!("→ 扫描 {} ...", repo.display());
    let stats = algo_station::problem_seed::seed_from_repo(&pool, &repo).await?;
    println!(
        "✓ 写入 {} 题、{} 标签；跳过 {} 个无 README、{} 个解析失败。",
        stats.problems, stats.tags, stats.skipped_no_readme, stats.skipped_parse_failed
    );
    Ok(())
}
