use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_artifact_cache(cache_path: &Path) -> PathBuf {
    cache_path.join("artifact-cache")
} 

pub fn get_atomic_sandbox_path(sandbox_path: &Path) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    sandbox_path.join(timestamp.to_string())
}

