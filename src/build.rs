use crate::helpers::{
    get_python_info, is_maturin_available, is_python_available, is_rust_python_project,
};
use serde_json;
use std::fs;
use std::process::{Command, Stdio};
use toml_edit::{value, DocumentMut};

/// Fetches the latest version of the package from PyPI, if online.
/// Returns `None` if the user is offline or if the package is not found.
fn fetch_latest_pypi_version(package_name: &str) -> Option<String> {
    let url = format!("https://pypi.org/pypi/{}/json", package_name);
    let response = Command::new("curl").arg("-s").arg(&url).output();

    if let Ok(output) = response {
        if output.status.success() {
            let json: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
            return json["info"]["version"].as_str().map(|s| s.to_string());
        }
    }
    None
}

/// Bumps the version in the specified file.
/// `level` can be "macro", "meso", or "micro".
fn bump_version(file_path: &str, force_bump: Option<&str>) -> Result<String, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Error reading file {}: {}", file_path, e))?;
    let mut doc = content
        .parse::<DocumentMut>()
        .map_err(|e| format!("Error parsing TOML: {}", e))?;

    // Extract project name and version
    // Check if this is a Rust or Python project
    let is_rust_project = file_path.ends_with("Cargo.toml");

    // Extract project name and version
    let name_key = if is_rust_project {
        "package"
    } else {
        "project"
    };
    let project_name = doc[name_key]["name"]
        .as_str()
        .ok_or(format!("No project name found in {}", file_path))?;
    let version = doc[name_key]["version"]
        .as_str()
        .ok_or("No version field found in TOML")?;

    // Check the latest PyPI version
    let mut should_bump = false;
    if let Some(latest_version) = fetch_latest_pypi_version(project_name) {
        if version == latest_version {
            should_bump = true;
            println!(
                "The current version ({}) is already published on PyPI.",
                version
            );
        } else {
            println!(
                "The current version ({}) is not on PyPI (latest: {}).",
                version, latest_version
            );
        }
    } else {
        println!("Skipping PyPI version check (offline or package not found).");
    }

    // Determine if we need to bump the version
    if !should_bump && force_bump.is_none() {
        println!("Version bump not required. Using version {}.", version);
        return Ok(version.to_string());
    }

    // If forced bump level is specified, use it; otherwise, bump micro by default
    let level = force_bump.unwrap_or("micro");
    // Split and bump the version
    let mut parts: Vec<u32> = version
        .split('.')
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect();

    // Ensure version is at least three parts long
    while parts.len() < 3 {
        parts.push(0);
    }

    // Bump the version
    match level {
        "macro" => parts[0] += 1,
        "meso" => parts[1] += 1,
        "micro" => parts[2] += 1,
        _ => return Err("Invalid version bump level".into()),
    }

    // Reset lower parts after a bump
    match level {
        "macro" => {
            parts[1] = 0;
            parts[2] = 0;
        }
        "meso" => {
            parts[2] = 0;
        }
        _ => {}
    }

    let new_version = format!("{}.{}.{}", parts[0], parts[1], parts[2]);
    doc["project"]["version"] = value(&new_version);

    fs::write(file_path, doc.to_string())
        .map_err(|e| format!("Error writing file {}: {}", file_path, e))?;

    Ok(new_version)
}

pub fn build_project(use_maturin: bool, bump_version_level: Option<&str>) {
    if use_maturin || is_rust_python_project() {
        let cargo_toml_path = "Cargo.toml";
        match bump_version(cargo_toml_path, bump_version_level) {
            Ok(new_version) => println!("Using version {}", new_version),
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        }
        build_with_maturin();
    } else {
        let pyproject_toml_path = "pyproject.toml";
        match bump_version(pyproject_toml_path, bump_version_level) {
            Ok(new_version) => println!("Using version {}", new_version),
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        }
        build_with_python();
    }
}

fn build_with_python() {
    if !is_python_available() {
        eprintln!("Error: `python` is not installed or not found in PATH.");
        return;
    }

    if let Some((python_path, python_version)) = get_python_info() {
        println!("Found {} at {}", python_version, python_path);
    } else {
        eprintln!("Error: Unable to determine Python path or version.");
        return;
    }

    println!("Building the Python project using `python -m build`...");
    let command = Command::new("python")
        .arg("-m")
        .arg("build")
        .arg("--no-isolation")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match command {
        Ok(mut child) => {
            let status = child.wait().expect("Failed to wait on child process");
            if status.success() {
                println!("Build successful!");
            } else {
                eprintln!(
                    "Build failed. If `build` is missing, install it with `pip install build`."
                );
            }
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
        }
    }
}

fn build_with_maturin() {
    if !is_maturin_available() {
        eprintln!("Error: `maturin` is not installed or not found in PATH.");
        return;
    }

    println!("Building the Python project using `maturin build --release`...");
    let command = Command::new("maturin")
        .args(&["build", "--release"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match command {
        Ok(mut child) => {
            let status = child.wait().expect("Failed to wait on child process");
            if status.success() {
                println!("Build successful!");
            } else {
                eprintln!("Build failed. Check the output above for details.");
            }
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
        }
    }
}
