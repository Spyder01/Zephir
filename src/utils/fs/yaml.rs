
use thiserror::Error;
use std::io;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use serde::{de::DeserializeOwned, Serialize};
use serde_yaml;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

pub async fn parse_yaml_from_file<T: DeserializeOwned>(path: &str) -> Result<T, ParseError> {
    let content = fs::read_to_string(path).await?;
    let parsed = serde_yaml::from_str(&content)?;
    Ok(parsed)
}

pub async fn write_yaml_to_file<T: Serialize>(path: &str, value: &T) -> Result<(), ParseError> {
    let serialized = serde_yaml::to_string(value)?;
    let mut file = fs::File::create(path).await?;
    file.write_all(serialized.as_bytes()).await?;
    Ok(())
}
