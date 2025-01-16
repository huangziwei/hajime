use std::process::{Command, Output};

pub fn build_project() {
    // Step 1: Check if `python3` is available
    if !is_python_available() {
        eprintln!("Error: `python3` is not installed or not found in PATH.");
        return;
    }

    // Step 2: Attempt to run `python3 -m build`
    println!("Building the Python project...");
    let build_output = Command::new("python3").arg("-m").arg("build").output();

    handle_command_output(
        build_output,
        "Build successful!",
        "Build failed. If `build` is missing, install it with `pip install build`.",
    );
}

// Helper function to check if `python3` is available
fn is_python_available() -> bool {
    Command::new("python")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// Helper function to handle command output
fn handle_command_output(
    output: Result<Output, std::io::Error>,
    success_msg: &str,
    failure_msg: &str,
) {
    match output {
        Ok(output) if output.status.success() => {
            println!("{}", success_msg);
        }
        Ok(output) => {
            eprintln!(
                "{}:\n{}",
                failure_msg,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
        }
    }
}
