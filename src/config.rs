use std::env;
use std::fs;
use std::path::PathBuf;
use toml::Value;

pub struct Config {
    pub excluded_files: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            excluded_files: Self::get_excluded_files(),
        }
    }

    /// Returns a vector of excluded file paths from the config file and some default paths.
    fn get_excluded_files() -> Vec<String> {
        let mut excluded_files = vec![
            "DS_Store".to_string(),
            "plugins/fish".to_string(),
            "plugins/zsh".to_string(),
        ];

        let config_path =
            PathBuf::from(env::var("CHEZMOI_FILES").unwrap_or_default()).join("config.toml");

        if let Ok(config) = fs::read_to_string(config_path) {
            if let Ok(config) = toml::from_str::<Value>(&config) {
                if let Some(files) = config.get("excluded_files").and_then(|v| v.as_array()) {
                    excluded_files.extend(
                        files
                            .iter()
                            .filter_map(|value| value.as_str().map(|s| s.to_string())),
                    );
                }
            }
        }

        excluded_files
    }
}
