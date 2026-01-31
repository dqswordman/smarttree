use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmarttreeError {
    #[error("failed to read config file at {path}: {source}")]
    ConfigRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse config file at {path}: {source}")]
    ConfigParse {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },
    #[error("invalid glob pattern '{pattern}': {source}")]
    InvalidPattern {
        pattern: String,
        #[source]
        source: globset::Error,
    },
}
