use anyhow::Error;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, path::Path};
use ftail::Ftail;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub project_format: String,
    pub plugin_format: String,
    pub plugin: String,
    pub update_rate: u64,
    pub app_id: i64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            project_format: "working on %%".to_string(),
            plugin_format: "%x %y open".to_string(),
            plugin: "OTT".to_string(),
            update_rate: 10,
            app_id: 1168141266517766175
        }
    }
}

fn load_config(config_path: &String) -> Result<Config, Error> {
    let config_file: String = fs::read_to_string(config_path).expect("error while reading config");
    let config: Config = serde_json::from_str(&config_file)?;

    Ok(config)
}

fn generate_config(config_path: &String) -> Result<(), std::io::Error> {
    let default_config = Config {
        ..Default::default()
    };
    fs::write(config_path, json!(default_config).to_string())
}

fn get_appdata_folder() -> String {
    format!("{}\\discordloops", std::env::var("APPDATA").unwrap())
}

pub fn setup() -> Result<Config, Error> {
    let our_folder = get_appdata_folder();
    let log_folder = format!("{}\\logs", our_folder);
    let config_file = format!("{}\\config.json", &our_folder);

    if !(Path::new(&our_folder).exists()) && !(Path::new(".\\config.json").exists()) {
        fs::create_dir_all(&log_folder)?;
        generate_config(&config_file)?;

        Ftail::new()
            .formatted_console(LevelFilter::Debug)
            .daily_file(&log_folder, LevelFilter::Error)
            .init()?;
    } else {
        Ftail::new()
            .formatted_console(LevelFilter::Debug)
            .single_file(".\\logs.txt", true, LevelFilter::Error)
            .init()?;
    }

    load_config(&config_file)
}