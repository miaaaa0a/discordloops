use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub project_format: String,
    pub plugin_format: String,
    pub plugin: String,
    pub update_rate: u64
}

pub fn load_config() -> std::result::Result<Config, serde_json::Error> {
    let config_path = String::from("config.json");
    let config_file: String = fs::read_to_string(config_path)
        .expect("error while reading config");
    let config: Config = serde_json::from_str(&config_file)?;

    return Ok(config);
}