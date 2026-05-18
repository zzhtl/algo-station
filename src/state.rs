use std::path::PathBuf;

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    /// 瘦身后的 doocs/leetcode 仓库根，运行时按需读取题面 markdown。
    pub leetcode_repo: PathBuf,
}
