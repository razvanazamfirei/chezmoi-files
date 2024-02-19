use serde::Deserialize;

use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "excluded-files")]
    pub excluded_files: FileList,
    #[serde(rename = "included-files")]
    pub included_files: FileList,
}

#[derive(Debug, Deserialize)]
pub struct FileList {
    pub files: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        let config_path =
            PathBuf::from(env::var("CHEZMOI_FILES").unwrap_or_default()).join("config.toml");

        let config = fs::read_to_string(config_path).unwrap_or_else(|_| String::new());

        let config: Config = toml::from_str(&config).unwrap_or_else(|_| Config {
            excluded_files: (FileList {
                files: vec![
                    "DS_Store".to_string(),
                    "plugins/fish".to_string(),
                    "plugins/zsh".to_string(),
                ],
            }),
            included_files: (FileList { files: vec![] }),
        });
        config
    }
}
