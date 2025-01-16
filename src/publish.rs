// src/publish.rs

use glob::glob;
use keyring::Entry;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

const SERVICE_NAME: &str = "hajime-cli";
const DEFAULT_ACCOUNT: &str = "default";

pub struct PyPiConfig {
    account: String,
}

impl PyPiConfig {
    pub fn new(account: Option<String>) -> Self {
        PyPiConfig {
            account: account.unwrap_or_else(|| DEFAULT_ACCOUNT.to_string()),
        }
    }

    fn keyring_entry(&self) -> Entry {
        Entry::new(SERVICE_NAME, &format!("pypi-token-{}", self.account)).expect("REASON")
    }

    fn get_token(&self) -> io::Result<String> {
        self.keyring_entry()
            .get_password()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    fn save_token(&self, token: &str) -> io::Result<()> {
        self.keyring_entry()
            .set_password(token)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    fn prompt_token(&self) -> io::Result<String> {
        println!("No token found for account '{}'", self.account);
        print!("Please enter your PyPI token: ");
        io::stdout().flush()?;

        let mut token = String::new();
        io::stdin().read_line(&mut token)?;
        Ok(token.trim().to_string())
    }

    pub fn update_token(&self) -> io::Result<()> {
        print!("Enter the new token for account '{}': ", self.account);
        io::stdout().flush()?;

        let mut token = String::new();
        io::stdin().read_line(&mut token)?;
        let token = token.trim();

        self.save_token(token)?;

        Ok(())
    }
}

pub fn publish_package(
    account: Option<String>,
    override_token: bool,
    use_maturin: bool,
) -> io::Result<()> {
    let config = PyPiConfig::new(account);

    if override_token {
        // Prompt user to update the token
        config.update_token()?;
        println!("Token for account '{}' has been updated.", config.account);
    }

    // Get token either from keyring or user input
    let token = match config.get_token() {
        Ok(token) => token,
        Err(_) => {
            let token = config.prompt_token()?;
            config.save_token(&token)?;
            token
        }
    };

    // Detect Rust-based Python project
    let is_rust_python_project =
        Path::new("Cargo.toml").exists() && Path::new("target/wheels").exists();

    if use_maturin || is_rust_python_project {
        // Find the `.whl` file in the target/wheels directory
        let mut wheel_path = None;
        for entry in glob("target/wheels/*.whl").expect("Failed to read glob pattern") {
            if let Ok(path) = entry {
                wheel_path = Some(path);
                break;
            }
        }

        if let Some(wheel_path) = wheel_path {
            println!("Using maturin to upload the package: {:?}", wheel_path);

            let command = Command::new("maturin")
                .args(&["upload", "-u", "__token__", "-p", &token])
                .arg(wheel_path)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn();

            match command {
                Ok(mut child) => {
                    let status = child.wait().expect("Failed to wait on child process");
                    if status.success() {
                        println!(
                            "Package published successfully using maturin and account '{}'!",
                            config.account
                        );
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "maturin upload failed. Check the output above for details.",
                        ));
                    }
                }
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Error running maturin: {}", e),
                    ));
                }
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No .whl file found in target/wheels. Please build the project first.",
            ));
        }
    } else {
        // Check if dist directory exists
        if !std::path::Path::new("dist").exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No dist directory found. Please run 'hajime build' first.",
            ));
        }
        // Run twine to publish the package and stream output
        let command = Command::new("twine")
            .args(&["upload", "dist/*"])
            .arg("--username")
            .arg("__token__") // PyPI uses `__token__` as the username for API tokens
            .arg("--password")
            .arg(&token) // Pass the actual token as the password
            .stdout(Stdio::inherit()) // Stream stdout to hajime's stdout
            .stderr(Stdio::inherit()) // Stream stderr to hajime's stderr
            .spawn();

        match command {
            Ok(mut child) => {
                let status = child.wait().expect("Failed to wait on child process");
                if status.success() {
                    println!(
                        "Package published successfully using account '{}'!",
                        config.account
                    );
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "twine upload failed. Check the output above for details.",
                    ));
                }
            }
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Error running twine: {}", e),
                ));
            }
        }
    }

    Ok(())
}
