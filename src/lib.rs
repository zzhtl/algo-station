use std::path::PathBuf;

pub mod articles_seed;
pub mod curriculum;
pub mod db;
pub mod error;
pub mod judge;
pub mod learning;
pub mod models;
pub mod problem_seed;
pub mod routes;
pub mod state;
pub mod static_assets;

/// 取当前可执行文件所在目录；拿不到则退回当前工作目录。
/// 作为运行时默认数据目录的基准。
pub fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn default_database_url() -> String {
    // 与 default_leetcode_repo 一致：优先 cwd 下的 data/（开发期/单仓部署），
    // 回退 exe 同级 data/（独立二进制部署）。
    let cwd_data = PathBuf::from("data");
    let db_path = if cwd_data.is_dir() {
        cwd_data.join("algo.db")
    } else {
        exe_dir().join("data").join("algo.db")
    };
    format!("sqlite://{}?mode=rwc", db_path.display())
}

/// 默认 doocs/leetcode 仓库路径：exe 同级 data/leetcode，开发期回退 ./data/leetcode。
pub fn default_leetcode_repo() -> PathBuf {
    let near_exe = exe_dir().join("data").join("leetcode");
    if near_exe.join("solution").is_dir() {
        return near_exe;
    }
    PathBuf::from("data/leetcode")
}
