use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub directories: Directories,
    pub theme: Theme,
    pub server: Server,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub domain: String,
    pub port: usize,
    pub root_path: String,
    pub images_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Directories {
    pub images: PathBuf,
    pub processed_images: PathBuf,
    pub output: PathBuf,
    pub templates: PathBuf,
    pub ignored_files: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    pub title: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_raw =
        read_to_string("config.toml").expect("No config.toml found in working directory.");
    let config: Config = toml::from_str(&config_raw)?;
    Ok(config)
}
