//! Integration tests for configuration file parsing.
//!
//! Tests various scenarios including malformed configs, missing files,
//! and custom configuration values.

use std::fs;
use std::io::Write as IoWrite;
use std::process::Command;

#[test]
fn test_malformed_config_falls_back_to_defaults() {
    let temp_dir = std::env::temp_dir().join(format!("chezmoi-test-{}", std::process::id()));
    let config_dir = temp_dir.join(".config").join("chezmoi");
    fs::create_dir_all(&config_dir).unwrap();

    let config_file = config_dir.join("chezmoi-files.toml");

    // Create malformed TOML
    fs::write(&config_file, "this is not valid [[[[ toml").unwrap();

    // Ensure the config file is flushed to disk
    fs::File::open(&config_file).unwrap();

    // Verify the file exists and is readable
    assert!(config_file.exists(), "Config file should exist");
    assert!(
        fs::read_to_string(&config_file).is_ok(),
        "Config file should be readable"
    );

    let temp_dir_str = temp_dir.to_str().expect("Failed to convert temp_dir to string");
    eprintln!("HOME will be set to: {temp_dir_str}");
    eprintln!("Config file path: {}", config_file.display());

    let mut child = Command::new(env!("CARGO_BIN_EXE_chezmoi-files"))
        .env("HOME", temp_dir_str)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"DS_Store\nregular.txt\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Debug output
    eprintln!("=== STDERR ===");
    eprintln!("{stderr}");
    eprintln!("=== STDOUT ===");
    eprintln!("{stdout}");
    eprintln!("=============");

    // DS_Store should be excluded by default config
    assert!(
        !stdout.contains("DS_Store"),
        "DS_Store should be excluded but appeared in output:\n{stdout}"
    );
    assert!(
        stdout.contains("regular.txt"),
        "regular.txt should be included but missing from output:\n{stdout}"
    );

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_config_with_only_colors_section() {
    let temp_dir = std::env::temp_dir().join(format!("chezmoi-test-{}", std::process::id()));
    let config_dir = temp_dir.join(".config").join("chezmoi");
    fs::create_dir_all(&config_dir).unwrap();

    let config_file = config_dir.join("chezmoi-files.toml");

    // Config with only colors section
    let config = r#"
[colors]
enabled = true
folder = "cyan"
"#;

    fs::write(&config_file, config).unwrap();

    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .env("HOME", &temp_dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"test.txt\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");

    // Should not panic and should output something
    assert!(output.status.success());

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_config_with_custom_extensions() {
    let temp_dir = std::env::temp_dir().join(format!("chezmoi-test-{}", std::process::id()));
    let config_dir = temp_dir.join(".config").join("chezmoi");
    fs::create_dir_all(&config_dir).unwrap();

    let config_file = config_dir.join("chezmoi-files.toml");

    // Config with custom extension colors
    let config = r#"
[excluded-files]
files = []

[included-files]
files = []

[colors]
enabled = true

[colors.extensions]
".test" = "red"
".custom" = "green"
"#;

    fs::write(&config_file, config).unwrap();

    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .env("HOME", &temp_dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"file.test\nfile.custom\n")
        .expect("Failed to write to stdin");
    let _ = stdin;

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("file.test"));
    assert!(stdout.contains("file.custom"));

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}
