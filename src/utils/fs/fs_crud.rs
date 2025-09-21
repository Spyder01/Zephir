use tokio::fs;
use std::path::Path;

pub async fn ensure_dir(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path).await
}

pub async fn dir_exists(path: &Path) -> bool {
    path.exists() && path.is_dir()
}
