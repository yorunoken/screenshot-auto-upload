use dirs;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};

use crate::Provider;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub key: String,
    pub provider: Provider,
    pub screenshot_watch_path: String,
}

impl Config {
    pub fn new(key: String, provider: Provider, screenshot_watch_path: String) -> Self {
        Config {
            screenshot_watch_path,
            provider,
            key,
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let config_text = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&config_text)?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        // Try to create every folder in the path
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

pub fn get_default_config_path() -> String {
    dirs::config_dir()
        .map(|config_dir| {
            config_dir
                .join("screenshot-auto-upload")
                .join("config.json")
        })
        .unwrap_or_else(|| Path::new("config.json").to_path_buf())
        .to_string_lossy()
        .to_string()
}
