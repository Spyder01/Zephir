use std::path::{Path, PathBuf};
use thiserror::Error;
use std::io;

use crate::models::config;
use crate::utils::fs::yaml;
use crate::compress::compress_zstd;

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Yaml(#[from] yaml::ParseError),
}

pub struct PackageEngine {
    directory_path: PathBuf,
    config_path: Option<PathBuf>,
}

impl PackageEngine {
    pub fn new(directory_path: &str, config_path: Option<&str>) -> Self {
        let dir_path = PathBuf::from(directory_path);

        let config_path_var = match config_path {
            Some(path) => Some(PathBuf::from(path)),
            None => None,
        };

        Self {
            directory_path: dir_path,
            config_path: config_path_var,
        }
    }

    pub async fn package(&self) -> Result<(), PackageError> {
        let parent_path = self.directory_path
                            .parent()
                            .expect("Invalid directory path.");

        let default_path = parent_path.join("zephir.yaml");
        let default_config = config::ZephirConfig::sane_defaults();
 
        if self.config_path.is_none() {
            yaml::write_yaml_to_file::<config::ZephirConfig>(default_path.to_str().expect("Invalid file-path."), &default_config).await?;
        }
         
        let config = self.config_path.as_ref().unwrap_or_else(|| &default_path);
        
        compress_zstd::compress_dir_to_zstd(self.directory_path
                .to_str()
                .expect("Invalid file-path."),
                parent_path
                .join(default_config.function.bundle.packagePath)
                .to_str()
                .expect("Invalid file-path."),
            1)?;


        Ok(())
    }
}

