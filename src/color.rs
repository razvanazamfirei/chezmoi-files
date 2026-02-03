//! Color scheme module for syntax-highlighted output.
//!
//! This module provides color schemes for different file types using ANSI escape codes.
//!
//! # Examples
//!
//! ```
//! use chezmoi_files::ColorScheme;
//!
//! // Create a new color scheme
//! let scheme = ColorScheme::new();
//!
//! // Print with colors
//! scheme.print_with_color("├──", "main.rs");
//!
//! // Create a scheme without colors
//! let no_color = ColorScheme::with_colors(false);
//! ```

use std::collections::HashMap;

/// A structure representing a color scheme.
///
/// This structure is used to represent a color scheme for the tree structure output.
pub struct ColorScheme {
    enabled: bool,
    reset: String,
    folder: String,
    default_file: String,
    extension_colors: HashMap<String, String>,
}

impl ColorScheme {
    /// Create a new color scheme with predefined colors.
    #[must_use]
    pub fn new() -> Self {
        Self::with_colors(true)
    }

    /// Create a color scheme with colors enabled or disabled.
    #[must_use]
    pub fn with_colors(enabled: bool) -> Self {
        if !enabled {
            return Self {
                enabled: false,
                reset: String::new(),
                folder: String::new(),
                default_file: String::new(),
                extension_colors: HashMap::new(),
            };
        }

        let mut extension_colors = HashMap::new();

        // Shell scripts
        for ext in [".fish", ".zsh", ".sh", ".nu", ".bash"] {
            extension_colors.insert(ext.to_string(), "\x1b[1;32m".to_string());
        }

        // Config files
        for ext in [".toml", ".json", ".yml", ".yaml", ".xml", ".ini", ".conf"] {
            extension_colors.insert(ext.to_string(), "\x1b[1;33m".to_string());
        }

        // Documentation
        for ext in [".md", ".txt", ".rst"] {
            extension_colors.insert(ext.to_string(), "\x1b[1;36m".to_string());
        }

        // Source code
        for ext in [
            ".rs", ".py", ".go", ".jl", ".js", ".ts", ".c", ".cpp", ".java",
        ] {
            extension_colors.insert(ext.to_string(), "\x1b[1;31m".to_string());
        }

        // Plists and other
        for ext in [".plist", ".sublime"] {
            extension_colors.insert(ext.to_string(), "\x1b[1;35m".to_string());
        }

        Self {
            enabled: true,
            reset: "\x1b[0m".to_string(),
            folder: "\x1b[1;37m".to_string(),
            default_file: "\x1b[1;34m".to_string(),
            extension_colors,
        }
    }

    /// Create a color scheme from custom color mappings.
    #[must_use]
    pub fn from_config(
        enabled: bool,
        folder: Option<String>,
        default_file: Option<String>,
        extension_colors: HashMap<String, String>,
    ) -> Self {
        if !enabled {
            return Self::with_colors(false);
        }

        let mut base = Self::new();

        if let Some(color) = folder {
            base.folder = Self::parse_color(&color);
        }

        if let Some(color) = default_file {
            base.default_file = Self::parse_color(&color);
        }

        for (ext, color) in extension_colors {
            base.extension_colors.insert(ext, Self::parse_color(&color));
        }

        base
    }

    /// Parse color names to ANSI codes.
    fn parse_color(color: &str) -> String {
        match color.to_lowercase().as_str() {
            "black" => "\x1b[1;30m".to_string(),
            "red" => "\x1b[1;31m".to_string(),
            "green" => "\x1b[1;32m".to_string(),
            "yellow" => "\x1b[1;33m".to_string(),
            "blue" => "\x1b[1;34m".to_string(),
            "magenta" => "\x1b[1;35m".to_string(),
            "cyan" => "\x1b[1;36m".to_string(),
            "white" => "\x1b[1;37m".to_string(),
            _ => color.to_string(), // Allow custom ANSI codes
        }
    }

    /// Returns the color code for a given file based on its extension.
    fn get_color_code_for_file(&self, name: &str) -> &str {
        if !self.enabled {
            return "";
        }

        for (ext, color) in &self.extension_colors {
            if name.ends_with(ext) {
                return color;
            }
        }

        &self.default_file
    }

    /// Prints a string with a color prefix based on the file type.
    ///
    /// Files without a dot in their name are treated as folders and colored accordingly.
    /// Files with extensions are colored based on their extension.
    pub fn print_with_color(&self, prefix: &str, name: &str) {
        if !self.enabled {
            println!("{prefix} {name}");
            return;
        }

        let color_code = if name.contains('.') {
            self.get_color_code_for_file(name)
        } else {
            &self.folder
        };

        println!("{prefix} {color_code}{name}{}", self.reset);
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_new() {
        let scheme = ColorScheme::new();
        assert!(scheme.enabled);
        assert!(!scheme.folder.is_empty());
        assert!(!scheme.default_file.is_empty());
    }

    #[test]
    fn test_color_scheme_disabled() {
        let scheme = ColorScheme::with_colors(false);
        assert!(!scheme.enabled);
        assert!(scheme.folder.is_empty());
        assert!(scheme.default_file.is_empty());
    }

    #[test]
    fn test_parse_color() {
        assert_eq!(ColorScheme::parse_color("red"), "\x1b[1;31m");
        assert_eq!(ColorScheme::parse_color("green"), "\x1b[1;32m");
        assert_eq!(ColorScheme::parse_color("blue"), "\x1b[1;34m");
        assert_eq!(ColorScheme::parse_color("custom"), "custom");
    }

    #[test]
    fn test_color_scheme_from_config() {
        let mut extensions = HashMap::new();
        extensions.insert(".rs".to_string(), "red".to_string());
        extensions.insert(".py".to_string(), "green".to_string());

        let scheme = ColorScheme::from_config(
            true,
            Some("white".to_string()),
            Some("blue".to_string()),
            extensions,
        );

        assert!(scheme.enabled);
        assert_eq!(scheme.folder, "\x1b[1;37m");
        assert_eq!(scheme.default_file, "\x1b[1;34m");
    }

    #[test]
    fn test_get_color_for_extension() {
        let scheme = ColorScheme::new();

        // Test that Rust files get the right color
        let color = scheme.get_color_code_for_file("main.rs");
        assert!(!color.is_empty());

        // Test that unknown extensions get default color
        let color = scheme.get_color_code_for_file("unknown.xyz");
        assert_eq!(color, scheme.default_file);
    }

    #[test]
    fn test_parse_all_colors() {
        assert_eq!(ColorScheme::parse_color("black"), "\x1b[1;30m");
        assert_eq!(ColorScheme::parse_color("red"), "\x1b[1;31m");
        assert_eq!(ColorScheme::parse_color("green"), "\x1b[1;32m");
        assert_eq!(ColorScheme::parse_color("yellow"), "\x1b[1;33m");
        assert_eq!(ColorScheme::parse_color("blue"), "\x1b[1;34m");
        assert_eq!(ColorScheme::parse_color("magenta"), "\x1b[1;35m");
        assert_eq!(ColorScheme::parse_color("cyan"), "\x1b[1;36m");
        assert_eq!(ColorScheme::parse_color("white"), "\x1b[1;37m");

        // Test case insensitivity
        assert_eq!(ColorScheme::parse_color("RED"), "\x1b[1;31m");
        assert_eq!(ColorScheme::parse_color("Green"), "\x1b[1;32m");

        // Test custom codes pass through
        assert_eq!(ColorScheme::parse_color("\x1b[1;90m"), "\x1b[1;90m");
        assert_eq!(ColorScheme::parse_color("custom"), "custom");
    }

    #[test]
    fn test_print_with_color_enabled() {
        let scheme = ColorScheme::new();
        // Should not panic
        scheme.print_with_color("├──", "test.rs");
        scheme.print_with_color("└──", "folder");
    }

    #[test]
    fn test_print_with_color_disabled() {
        let scheme = ColorScheme::with_colors(false);
        // Should not panic and output without colors
        scheme.print_with_color("├──", "test.txt");
        scheme.print_with_color("└──", "dir");
    }

    #[test]
    fn test_color_scheme_with_all_extensions() {
        let scheme = ColorScheme::new();

        // Shell scripts
        assert!(!scheme.get_color_code_for_file("script.sh").is_empty());
        assert!(!scheme.get_color_code_for_file("script.bash").is_empty());
        assert!(!scheme.get_color_code_for_file("script.zsh").is_empty());
        assert!(!scheme.get_color_code_for_file("script.fish").is_empty());
        assert!(!scheme.get_color_code_for_file("script.nu").is_empty());

        // Config files
        assert!(!scheme.get_color_code_for_file("config.toml").is_empty());
        assert!(!scheme.get_color_code_for_file("config.json").is_empty());
        assert!(!scheme.get_color_code_for_file("config.yml").is_empty());
        assert!(!scheme.get_color_code_for_file("config.yaml").is_empty());
        assert!(!scheme.get_color_code_for_file("config.xml").is_empty());
        assert!(!scheme.get_color_code_for_file("config.ini").is_empty());
        assert!(!scheme.get_color_code_for_file("config.conf").is_empty());

        // Documentation
        assert!(!scheme.get_color_code_for_file("README.md").is_empty());
        assert!(!scheme.get_color_code_for_file("notes.txt").is_empty());
        assert!(!scheme.get_color_code_for_file("docs.rst").is_empty());

        // Source code
        assert!(!scheme.get_color_code_for_file("main.rs").is_empty());
        assert!(!scheme.get_color_code_for_file("script.py").is_empty());
        assert!(!scheme.get_color_code_for_file("app.js").is_empty());
        assert!(!scheme.get_color_code_for_file("app.ts").is_empty());
        assert!(!scheme.get_color_code_for_file("main.go").is_empty());
        assert!(!scheme.get_color_code_for_file("program.c").is_empty());
        assert!(!scheme.get_color_code_for_file("program.cpp").is_empty());
        assert!(!scheme.get_color_code_for_file("Main.java").is_empty());
        assert!(!scheme.get_color_code_for_file("script.jl").is_empty());

        // Other
        assert!(!scheme.get_color_code_for_file("Info.plist").is_empty());
        assert!(
            !scheme
                .get_color_code_for_file("settings.sublime")
                .is_empty()
        );
    }

    #[test]
    fn test_color_scheme_folder_vs_file() {
        let scheme = ColorScheme::new();

        // Files with dots get file colors
        scheme.print_with_color("", "test.txt");

        // Files without dots get folder colors
        scheme.print_with_color("", "folder");
    }

    #[test]
    fn test_from_config_disabled() {
        let scheme = ColorScheme::from_config(false, None, None, HashMap::new());
        assert!(!scheme.enabled);
        assert_eq!(scheme.get_color_code_for_file("test.rs"), "");
    }

    #[test]
    fn test_from_config_custom_colors() {
        let mut extensions = HashMap::new();
        extensions.insert(".test".to_string(), "red".to_string());

        let scheme = ColorScheme::from_config(
            true,
            Some("cyan".to_string()),
            Some("magenta".to_string()),
            extensions,
        );

        assert_eq!(scheme.folder, "\x1b[1;36m");
        assert_eq!(scheme.default_file, "\x1b[1;35m");
    }

    #[test]
    fn test_default_trait() {
        let scheme1 = ColorScheme::default();
        let scheme2 = ColorScheme::new();

        assert_eq!(scheme1.enabled, scheme2.enabled);
        assert_eq!(scheme1.folder, scheme2.folder);
    }
}
