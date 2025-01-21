use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn check_package() -> io::Result<()> {
    // Determine the directory to check (dist or target/wheels)
    let dist_dir = if Path::new("dist").exists() {
        "dist/*"
    } else if Path::new("target/wheels").exists() {
        "target/wheels/*"
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No build artifacts found. Please build the project first.",
        ));
    };

    println!("Running twine check on {}...", dist_dir);

    // Run twine check on the determined directory
    let command = Command::new("uv")
        .args(&["run", "twine", "check", dist_dir])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match command {
        Ok(mut child) => {
            let status = child.wait()?;
            if status.success() {
                println!("Twine check passed successfully!");
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Twine check failed. Check the output above for details.",
                ));
            }
        }
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error running twine check: {}", e),
            ));
        }
    }

    Ok(())
}
