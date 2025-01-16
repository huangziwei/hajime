use std::process::{Command, Stdio};

pub fn build_project() {
    // Step 1: Check if `python3` is available
    if !is_python_available() {
        eprintln!("Error: `python3` is not installed or not found in PATH.");
        return;
    }

    // Step 2: Attempt to run `python3 -m build`
    println!("Building the Python project...");
    // Use Command to execute `python3 -m build` and stream output
    let command = Command::new("python3")
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

// Helper function to check if `python3` is available
fn is_python_available() -> bool {
    Command::new("python")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
