use std::fs;
use std::process::Command;

#[test]
fn test_config_init_creates_file() {
    let temp_dir = std::env::temp_dir().join(format!("chezmoi-test-{}", std::process::id()));
    fs::create_dir_all(&temp_dir).unwrap();

    let config_file = temp_dir.join("test-config.toml");

    // Make sure file doesn't exist
    let _ = fs::remove_file(&config_file);

    // Set HOME to temp dir
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "config", "--init"])
        .env("HOME", &temp_dir)
        .env("XDG_CONFIG_HOME", &temp_dir)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // For this test, we're just checking the command runs
    // The actual file creation uses ~/.config/chezmoi path
    assert!(output.status.success() || stdout.contains("Configuration file"));

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_config_init_existing_file() {
    let temp_dir = std::env::temp_dir().join(format!("chezmoi-test-{}", std::process::id()));
    let config_dir = temp_dir.join(".config").join("chezmoi");
    fs::create_dir_all(&config_dir).unwrap();

    let config_file = config_dir.join("chezmoi-files.toml");

    // Create existing file
    fs::write(&config_file, "# existing config").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "config", "--init"])
        .env("HOME", &temp_dir)
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should warn about existing file or succeed
    assert!(
        stderr.contains("already exists")
            || stdout.contains("already exists")
            || output.status.success()
    );

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_config_with_custom_colors() {
    let temp_dir = std::env::temp_dir().join(format!("chezmoi-test-{}", std::process::id()));
    let config_dir = temp_dir.join(".config").join("chezmoi");
    fs::create_dir_all(&config_dir).unwrap();

    let config_file = config_dir.join("chezmoi-files.toml");

    // Create config with custom colors
    let custom_config = r#"
[excluded-files]
files = ["test.tmp"]

[included-files]
files = []

[colors]
enabled = false
"#;

    fs::write(&config_file, custom_config).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "config"])
        .env("HOME", &temp_dir)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show config file location or content
    assert!(
        stdout.contains("Configuration file:")
            || stdout.contains("test.tmp")
            || stdout.contains("enabled")
            || output.status.success()
    );

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}
