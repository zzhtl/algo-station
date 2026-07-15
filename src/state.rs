use std::path::PathBuf;
use std::sync::Arc;

use sqlx::SqlitePool;

use crate::curriculum::CurriculumCatalog;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    /// 瘦身后的 doocs/leetcode 仓库根，运行时按需读取题面 markdown。
    pub leetcode_repo: PathBuf,
    pub catalog: Arc<CurriculumCatalog>,
}
