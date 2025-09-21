use tokio::fs;
use std::path::Path;

pub async fn ensure_dir(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path).await
}

pub async fn dir_exists(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

pub fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            std::fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

