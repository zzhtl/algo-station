use std::path::Path;

use anyhow::Result;

pub async fn ensure_database_file(database_url: &str) -> Result<()> {
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        let path = path.split('?').next().unwrap_or(path);
        if path != ":memory:" {
            if let Some(parent) = Path::new(path).parent() {
                if !parent.as_os_str().is_empty() {
                    tokio::fs::create_dir_all(parent).await?;
                }
            }
            if !Path::new(path).exists() {
                tokio::fs::File::create(path).await?;
            }
        }
    }
    Ok(())
}
