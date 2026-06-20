use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub project_path: String,
    pub token: Option<String>,
    pub api_url: Option<String>,
}

fn get_config_path() -> PathBuf {
    let mut path = if let Ok(home) = std::env::var("USERPROFILE") {
        PathBuf::from(home)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
    } else {
        PathBuf::from(".")
    };
    path.push(".github-cli-config.toml");
    path
}

impl Config {
    pub fn load() -> Self {
        let config_path = get_config_path();
        
        let mut config = if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(c) = toml::from_str::<Config>(&content) {
                c
            } else {
                Self::default_config()
            }
        } else {
            Self::default_config()
        };

        // Dynamically bind project_path to the current working directory
        config.project_path = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string());

        // Make sure the global config directory/file is saved
        let _ = Self::save(&config);
        config
    }

    fn default_config() -> Self {
        Self {
            project_path: ".".to_string(),
            token: None,
            api_url: Some("http://localhost:5000/api".to_string()),
        }
    }

    pub fn save(config: &Config) -> Result<(), std::io::Error> {
        let config_path = get_config_path();
        let content = toml::to_string(config).unwrap();
        fs::write(config_path, content)
    }
}
