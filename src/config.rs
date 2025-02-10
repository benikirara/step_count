use serde::Deserialize;
use std::fs;
use std::path::Path;

///config.json用構造体
#[derive(Deserialize, Debug)]
pub struct Config {
    pub included_extensions: Vec<String>,
    pub exclude_files: Vec<String>,
}

pub enum ConfigError {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl Config {
    /// config.jsonを読み込み、Configインスタンスを生成する
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        use ConfigError::*;
        let config_contents = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(io_error) => return Err(IoError(io_error)),
        };
        let config: Config = match serde_json::from_str(&config_contents) {
            Ok(config) => config,
            Err(serde_error) => return Err(SerdeError(serde_error)),
        };
        Ok(config)
    }
}
