//! Configuration module for file filtering.
//!
//! This module handles loading and parsing configuration from a TOML file
//! specified by the `CHEZMOI_FILES` environment variable.

use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Configuration for file filtering.
///
/// This struct contains lists of files to exclude and include when processing paths.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// List of files to exclude from the tree visualization.
    #[serde(rename = "excluded-files")]
    pub excluded_files: FileList,
    /// List of files to include (overrides exclusions).
    #[serde(rename = "included-files")]
    pub included_files: FileList,
}

/// A list of file patterns.
#[derive(Debug, Deserialize)]
pub struct FileList {
    /// The file patterns to match against.
    pub files: Vec<String>,
}

impl Config {
    /// Creates a new `Config` by loading from the configuration file.
    ///
    /// The configuration file path is determined by the `CHEZMOI_FILES` environment
    /// variable. If the file doesn't exist or cannot be parsed, default values are used.
    ///
    /// # Default Values
    ///
    /// - Excluded files: `DS_Store`, `plugins/fish`, `plugins/zsh`
    /// - Included files: (empty)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use chezmoi_files::Config;
    ///
    /// let config = Config::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let config_path =
            PathBuf::from(env::var("CHEZMOI_FILES").unwrap_or_default()).join("config.toml");

        let config = fs::read_to_string(config_path).unwrap_or_else(|_| String::new());

        toml::from_str(&config).unwrap_or_else(|_| Self::default_config())
    }

    /// Returns the default configuration.
    fn default_config() -> Self {
        Self {
            excluded_files: FileList {
                files: vec![
                    String::from("DS_Store"),
                    String::from("plugins/fish"),
                    String::from("plugins/zsh"),
                ],
            },
            included_files: FileList { files: vec![] },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
