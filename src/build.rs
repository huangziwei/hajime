use crate::helpers::{get_python_info, is_maturin_available, is_python_available};
use std::process::{Command, Stdio};
pub fn build_project(use_maturin: bool) {
    if use_maturin {
        build_with_maturin();
    } else {
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
        .stdout(Stdio::inherit()) // Stream stdout to hajime's stdout
        .stderr(Stdio::inherit()) // Stream stderr to hajime's stderr
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
        .stdout(Stdio::inherit()) // Stream stdout to hajime's stdout
        .stderr(Stdio::inherit()) // Stream stderr to hajime's stderr
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
