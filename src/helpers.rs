use std::fs;
use std::path::PathBuf;
use std::process::Command;

// Helper function to check if `python` is available
pub fn is_python_available() -> bool {
    Command::new("python")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// Helper function to check if `maturin` is available
pub fn is_maturin_available() -> bool {
    Command::new("maturin")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// Helper function to get Python executable path and version
pub fn get_python_info() -> Option<(String, String)> {
    // Get the path to the Python executable
    let python_path = Command::new("which")
        .arg("python")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })?;

    // Get the Python version
    let python_version = Command::new(&python_path)
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })?;

    // Return both the path and the version
    Some((python_path, python_version))
}

pub fn get_latest_wheel_file(directory: &str) -> Option<PathBuf> {
    let mut files: Vec<_> = fs::read_dir(directory)
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "whl"))
        .collect();

    files.sort_by_key(|entry| entry.metadata().and_then(|meta| meta.modified()).ok());
    files.last().map(|entry| entry.path())
}
