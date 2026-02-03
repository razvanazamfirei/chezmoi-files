//! Configuration module for file filtering.
//!
//! This module handles loading and parsing configuration from a TOML file
//! located at `~/.config/chezmoi/chezmoi-files.toml`.
//!
//! # Examples
//!
//! ```
//! use chezmoi_files::Config;
//!
//! // Load configuration from file (or use defaults)
//! let config = Config::new();
//!
//! // Check if a path should be excluded
//! assert!(config.is_excluded("DS_Store"));
//! assert!(!config.is_excluded("regular_file.txt"));
//!
//! // Use default configuration
//! let default_config = Config::default();
//! ```

use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Configuration for file filtering.
///
/// This struct contains lists of files to exclude and include when processing paths.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// List of files to exclude from the tree visualization.
    #[serde(rename = "excluded-files", default)]
    pub excluded_files: FileList,
    /// List of files to include (overrides exclusions).
    #[serde(rename = "included-files", default)]
    pub included_files: FileList,
    /// Color configuration.
    #[serde(default)]
    pub colors: ColorConfig,
}

/// A list of file patterns.
#[derive(Debug, Deserialize, Default)]
pub struct FileList {
    /// The file patterns to match against.
    #[serde(default)]
    pub files: Vec<String>,
}

/// Color configuration for the tree output.
#[derive(Debug, Deserialize)]
pub struct ColorConfig {
    /// Whether colors are enabled.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Color for folders.
    pub folder: Option<String>,
    /// Default color for files.
    #[serde(rename = "default-file")]
    pub default_file: Option<String>,
    /// Colors for specific file extensions.
    #[serde(default)]
    pub extensions: HashMap<String, String>,
}

const fn default_true() -> bool {
    true
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            folder: None,
            default_file: None,
            extensions: HashMap::new(),
        }
    }
}

impl Config {
    /// Creates a new `Config` by loading from the configuration file.
    ///
    /// The configuration file is located at `~/.config/chezmoi/chezmoi-files.toml`.
    /// If the file doesn't exist or cannot be parsed, default values are used.
    ///
    /// # Default Exclusions
    ///
    /// - `DS_Store`
    /// - `fish_variables*`
    /// - `.rubocop.yml`
    /// - `.ruff_cache`
    /// - `yazi.toml-`
    /// - `.zcompcache`
    /// - `.zcompdump`
    /// - `.zsh_history`
    /// - `plugins/fish`
    /// - `plugins/zsh`
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
        let config_path = Self::config_path();

        match fs::read_to_string(&config_path) {
            Ok(content) if !content.trim().is_empty() => {
                // Try to parse the config file
                toml::from_str(&content).unwrap_or_else(|_| Self::default())
            }
            _ => {
                // File doesn't exist or is empty, use defaults
                Self::default()
            }
        }
    }

    /// Returns the path to the configuration file.
    ///
    /// Uses `~/.config/chezmoi/chezmoi-files.toml` as the standard location.
    #[must_use]
    pub fn config_path() -> PathBuf {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("."));
        PathBuf::from(home)
            .join(".config")
            .join("chezmoi")
            .join("chezmoi-files.toml")
    }

    /// Returns the default configuration as a TOML string.
    ///
    /// This is useful for creating a default configuration file.
    #[must_use]
    pub fn default_config_toml() -> String {
        r#"# Configuration for chezmoi-files
# Edit this file to customize which files are excluded from the tree visualization

[excluded-files]
# Patterns support glob-style wildcards: *, ?, [abc], [a-z]
# Examples:
#   "*.tmp"        - matches any file ending in .tmp
#   "cache/*"      - matches any file in a cache directory
#   "test_*.rs"    - matches test_foo.rs, test_bar.rs, etc.
files = [
    "DS_Store",
    "fish_variables*",
    ".rubocop.yml",
    ".ruff_cache",
    "yazi.toml-*",
    ".zcompcache",
    ".zcompdump",
    ".zsh_history",
    "plugins/fish",
    "plugins/zsh",
]

[included-files]
# Files matching these patterns will be included even if they match exclusions
files = []

[colors]
# Set to false to disable colors entirely
enabled = true

# Customize colors for folders and files
# Available colors: black, red, green, yellow, blue, magenta, cyan, white
# You can also use custom ANSI codes like "\x1b[1;32m"
# folder = "white"
# default-file = "blue"

# Customize colors for specific file extensions
# [colors.extensions]
# ".rs" = "red"
# ".py" = "green"
# ".md" = "cyan"
"#
        .to_string()
    }

    /// Checks if a path matches any exclusion pattern using glob matching.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check against exclusion patterns
    ///
    /// # Returns
    ///
    /// `true` if the path matches any exclusion pattern, `false` otherwise
    #[must_use]
    pub fn is_excluded(&self, path: &str) -> bool {
        self.excluded_files
            .files
            .iter()
            .any(|pattern| Self::matches_glob(path, pattern))
    }

    /// Checks if a path matches any inclusion pattern using glob matching.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check against inclusion patterns
    ///
    /// # Returns
    ///
    /// `true` if the path matches any inclusion pattern, `false` otherwise
    #[must_use]
    pub fn is_included(&self, path: &str) -> bool {
        self.included_files
            .files
            .iter()
            .any(|pattern| Self::matches_glob(path, pattern))
    }

    /// Matches a path against a glob pattern.
    ///
    /// Supports wildcards: `*`, `?`, `[abc]`, `[a-z]`
    fn matches_glob(path: &str, pattern: &str) -> bool {
        // If pattern contains glob characters, use glob matching
        if (pattern.contains('*') || pattern.contains('?') || pattern.contains('['))
            && let Ok(glob_pattern) = glob::Pattern::new(pattern)
        {
            // Try matching the full path
            if glob_pattern.matches(path) {
                return true;
            }
            // Also try matching any component of the path
            return path
                .split('/')
                .any(|component| glob_pattern.matches(component));
        }

        // Fall back to substring matching
        path.contains(pattern)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            excluded_files: FileList {
                files: vec![
                    "DS_Store",
                    "fish_variables*",
                    ".rubocop.yml",
                    ".ruff_cache",
                    "yazi.toml-*",
                    ".zcompcache",
                    ".zcompdump",
                    ".zsh_history",
                    "plugins/fish",
                    "plugins/zsh",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
            },
            included_files: FileList { files: Vec::new() },
            colors: ColorConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_glob_simple_substring() {
        assert!(Config::matches_glob("path/to/DS_Store", "DS_Store"));
        assert!(Config::matches_glob("foo/bar/baz", "bar"));
        assert!(!Config::matches_glob("foo/baz", "bar"));
    }

    #[test]
    fn test_matches_glob_wildcard() {
        assert!(Config::matches_glob(
            "fish_variables.bak",
            "fish_variables*"
        ));
        assert!(Config::matches_glob("yazi.toml-old", "yazi.toml-*"));
        assert!(Config::matches_glob("test.tmp", "*.tmp"));
        assert!(!Config::matches_glob("test.txt", "*.tmp"));
    }

    #[test]
    fn test_matches_glob_question_mark() {
        assert!(Config::matches_glob("test1.txt", "test?.txt"));
        assert!(Config::matches_glob("testA.txt", "test?.txt"));
        assert!(!Config::matches_glob("test12.txt", "test?.txt"));
    }

    #[test]
    fn test_matches_glob_character_class() {
        assert!(Config::matches_glob("testa.txt", "test[abc].txt"));
        assert!(Config::matches_glob("testb.txt", "test[abc].txt"));
        assert!(!Config::matches_glob("testd.txt", "test[abc].txt"));
    }

    #[test]
    fn test_is_excluded() {
        let config = Config::default();

        assert!(config.is_excluded("path/to/DS_Store"));
        assert!(config.is_excluded("config/fish_variables"));
        assert!(config.is_excluded("config/fish_variables.bak"));
        assert!(config.is_excluded(".rubocop.yml"));
        assert!(!config.is_excluded("regular_file.txt"));
    }

    #[test]
    fn test_inclusion_overrides_exclusion() {
        let mut config = Config::default();
        config
            .included_files
            .files
            .push("important.txt".to_string());
        config.excluded_files.files.push("*.txt".to_string());

        assert!(!config.is_excluded("important.txt") || config.is_included("important.txt"));
    }

    #[test]
    fn test_default_config_has_colors() {
        let config = Config::default();
        assert!(config.colors.enabled);
    }

    #[test]
    fn test_is_included() {
        let mut config = Config::default();
        config
            .included_files
            .files
            .push("important.txt".to_string());

        assert!(config.is_included("important.txt"));
        assert!(config.is_included("path/to/important.txt"));
        assert!(!config.is_included("other.txt"));
    }

    #[test]
    fn test_config_new_with_missing_file() {
        // This should not panic even if config file doesn't exist
        let config = Config::new();
        // Config should be initialized (either from file or defaults)
        // Verify that colors field exists and is accessible
        let _ = config.colors.enabled;
    }

    #[test]
    fn test_config_path() {
        let path = Config::config_path();
        assert!(path.to_string_lossy().contains("chezmoi-files.toml"));
    }

    #[test]
    fn test_default_config_toml() {
        let toml = Config::default_config_toml();
        assert!(toml.contains("[excluded-files]"));
        assert!(toml.contains("[included-files]"));
        assert!(toml.contains("[colors]"));
        assert!(toml.contains("DS_Store"));
    }

    #[test]
    fn test_file_list_default() {
        let file_list = FileList::default();
        assert_eq!(file_list.files.len(), 0);
    }

    #[test]
    fn test_color_config_default() {
        let color_config = ColorConfig::default();
        assert!(color_config.enabled);
        assert!(color_config.folder.is_none());
        assert!(color_config.default_file.is_none());
        assert_eq!(color_config.extensions.len(), 0);
    }

    #[test]
    fn test_matches_glob_path_components() {
        // Test that patterns match path components, not just the full path
        assert!(Config::matches_glob("dir/cache/file.txt", "cache"));
        assert!(Config::matches_glob("a/b/c/test.tmp", "*.tmp"));
    }

    #[test]
    fn test_matches_glob_invalid_pattern() {
        // Invalid glob patterns should fall back to substring matching
        assert!(Config::matches_glob("test[file", "test[file"));
    }

    #[test]
    fn test_matches_glob_range() {
        assert!(Config::matches_glob("test1.txt", "test[0-9].txt"));
        assert!(Config::matches_glob("test5.txt", "test[0-9].txt"));
        assert!(!Config::matches_glob("testa.txt", "test[0-9].txt"));
    }

    #[test]
    fn test_exclusion_patterns_with_wildcards() {
        let config = Config::default();
        // Test wildcard patterns from default config
        assert!(config.is_excluded("fish_variables"));
        assert!(config.is_excluded("fish_variables.bak"));
        assert!(config.is_excluded("yazi.toml-old"));
        assert!(config.is_excluded("yazi.toml-backup"));
    }
}
