use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub watched_directories: Vec<PathBuf>,
    pub data_shards: usize,
    pub parity_shards: usize,
}

pub fn load_config(path: &str) -> Result<AppConfig> {
    let config_str = fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&config_str)?;
    Ok(config)
} 