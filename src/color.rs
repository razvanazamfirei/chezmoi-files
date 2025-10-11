//! Color scheme module for syntax-highlighted output.
//!
//! This module provides color schemes for different file types using ANSI escape codes.

/// A structure representing a color scheme.
///
/// This structure is used to represent a color scheme for the tree structure output.
///
/// # Fields
///
/// * `reset` - The reset color code.
/// * `folder` - The color code for folders.
/// * `default_file` - The default color code for files.
/// * `file_colors` - The color codes for specific file extensions.
pub struct ColorScheme {
    reset: &'static str,
    folder: &'static str,
    default_file: &'static str,
    file_colors: &'static [(&'static [&'static str], &'static str)],
}

impl ColorScheme {
    /// Create a new color scheme with predefined colors.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            reset: "\x1b[0m",
            folder: "\x1b[1;37m",
            default_file: "\x1b[1;34m",
            file_colors: &[
                (&[".fish", ".zsh", ".sh", ".nu"], "\x1b[1;32m"),
                (
                    &[".toml", ".json", ".yml", ".yaml", ".xml", ".ini", ".conf"],
                    "\x1b[1;33m",
                ),
                (&[".md", ".txt"], "\x1b[1;36m"),
                (&[".rs", ".py", ".go", ".jl"], "\x1b[1;31m"),
                (&[".plist", ".sublime"], "\x1b[1;35m"),
            ],
        }
    }

    /// Returns the color code for a given file based on its extension.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the file.
    ///
    /// # Returns
    ///
    /// A string slice that represents the color code for the file.
    fn get_color_code_for_file(&self, name: &str) -> &'static str {
        self.file_colors
            .iter()
            .find(|&&(extensions, _)| extensions.iter().any(|extension| name.ends_with(extension)))
            .map_or(self.default_file, |&(_, color)| color)
    }

    /// Prints a string with a color prefix based on the file type.
    ///
    /// Files without a dot in their name are treated as folders and colored accordingly.
    /// Files with extensions are colored based on their extension.
    ///
    /// # Arguments
    ///
    /// * `prefix` - A string slice that holds the prefix to be printed.
    /// * `name` - A string slice that holds the name of the file or folder.
    pub fn print_with_color(&self, prefix: &str, name: &str) {
        let color_code = if name.contains('.') {
            self.get_color_code_for_file(name)
        } else {
            self.folder
        };

        println!("{prefix} {color_code}{name}{}", self.reset);
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::new()
    }
}
