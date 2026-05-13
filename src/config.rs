use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub project_path: String,
}

impl Config {
    pub fn load() -> Self {
        let config_path = "config.toml";
        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(config) = toml::from_str::<Config>(&content) {
                return config;
            }
        }

        // Default config if file doesn't exist or is invalid
        let default_config = Config {
            project_path: std::env::var("PROJECT_PATH").unwrap_or_else(|_| ".".to_string()),
        };
        
        let _ = Self::save(&default_config);
        default_config
    }

    pub fn save(config: &Config) -> Result<(), std::io::Error> {
        let content = toml::to_string(config).unwrap();
        fs::write("config.toml", content)
    }
}
