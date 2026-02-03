//! Integration tests for the main CLI functionality.
//!
//! Tests tree output, colorization, sorting, statistics, and filtering behavior.

use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_basic_tree_output() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"src/main.rs\nsrc/config.rs\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("src"));
    assert!(stdout.contains("main.rs"));
    assert!(stdout.contains("config.rs"));
}

#[test]
fn test_no_color_flag() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--", "--no-color"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"test.rs\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should not contain ANSI color codes
    assert!(!stdout.contains("\x1b["));
}

#[test]
fn test_stats_flag() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--", "--stats"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"src/main.rs\nsrc/config.rs\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Files:"));
    assert!(stdout.contains("Directories:"));
    assert!(stdout.contains("Excluded:"));
}

#[test]
fn test_sort_name() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--", "--sort", "name", "--no-color"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"c.txt\na.txt\nb.txt\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check that files appear in alphabetical order
    let a_pos = stdout.find("a.txt").unwrap();
    let b_pos = stdout.find("b.txt").unwrap();
    let c_pos = stdout.find("c.txt").unwrap();

    assert!(a_pos < b_pos && b_pos < c_pos);
}

#[test]
fn test_config_subcommand() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "config", "--default"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("[excluded-files]"));
    assert!(stdout.contains("[included-files]"));
    assert!(stdout.contains("[colors]"));
}

#[test]
fn test_config_show() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "config"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Configuration file:"));
}

#[test]
fn test_version_flag() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("chezmoi-files"));
}

#[test]
fn test_help_flag() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("Options:"));
}

#[test]
fn test_empty_input() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--", "--no-color"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"").expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should just show root
    assert!(stdout.contains('.'));
}

#[test]
fn test_excluded_files() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--", "--stats", "--no-color"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"DS_Store\nregular.txt\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // DS_Store should be excluded
    assert!(!stdout.contains("DS_Store"));
    assert!(stdout.contains("regular.txt"));
    assert!(stdout.contains("Excluded: 1"));
}

#[test]
fn test_sort_type() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--", "--sort", "type", "--no-color"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"file.txt\ndir/nested.txt\nfile.rs\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Directory should appear before files
    let dir_pos = stdout.find("dir").unwrap();
    let file_txt_pos = stdout.find("file.txt").unwrap_or(usize::MAX);
    let file_rs_pos = stdout.find("file.rs").unwrap_or(usize::MAX);

    assert!(dir_pos < file_txt_pos || dir_pos < file_rs_pos);
}

#[test]
fn test_nested_paths() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--", "--no-color"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"a/b/c/d/file.txt\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains('a'));
    assert!(stdout.contains('b'));
    assert!(stdout.contains('c'));
    assert!(stdout.contains('d'));
    assert!(stdout.contains("file.txt"));
}

#[test]
fn test_multiple_files_same_dir() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--", "--stats", "--no-color"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"src/a.rs\nsrc/b.rs\nsrc/c.rs\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("a.rs"));
    assert!(stdout.contains("b.rs"));
    assert!(stdout.contains("c.rs"));
    assert!(stdout.contains("Files: 3"));
    assert!(stdout.contains("Directories: 1"));
}

#[test]
fn test_glob_pattern_exclusion() {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--", "--stats", "--no-color"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"fish_variables\nfish_variables.bak\nregular.txt\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Both fish_variables files should be excluded due to wildcard pattern
    assert!(!stdout.contains("fish_variables"));
    assert!(stdout.contains("regular.txt"));
    assert!(stdout.contains("Excluded: 2"));
}
