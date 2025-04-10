use crate::helpers::{is_git_installed, is_uv_installed, to_snake_case};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Creates a new Python project skeleton
///
/// # Arguments
/// * `project_name` - The name of the project to create.
/// * `force` - A boolean indicating whether to overwrite an existing project.
pub fn create_project(project_name: &str, force: bool) -> std::io::Result<()> {
    let base_path = Path::new(project_name);
    let source_name = to_snake_case(project_name);
    let source_path = base_path.join(&source_name);
    let venv_path: std::path::PathBuf = base_path.join(".venv");

    // Check if the project directory already exists
    if base_path.exists() {
        if !force {
            eprintln!(
                "Error: A project with the name '{}' already exists. Use the --force flag to overwrite.",
                project_name
            );
            return Ok(());
        } else {
            println!("Warning: Overwriting existing project '{}'.", project_name);
            fs::remove_dir_all(&base_path)?; // Remove the existing directory if forced
        }
    }

    // Create the base project directory
    fs::create_dir_all(&base_path)?;

    // Create the source directory inside the base path with `__init__.py` and `main.py`
    fs::create_dir_all(&source_path)?;

    // Create `__init__.py` (empty)
    File::create(source_path.join("__init__.py"))?;

    // Create `main.py` with a "Hello, world!" example
    let mut main_py = File::create(source_path.join("greet.py"))?;
    writeln!(main_py, "def hello(name: str = \"world\") -> None:")?;
    writeln!(main_py, "    print(f\"Hello, {{name}}!\")")?;
    writeln!(main_py, "\n\nif __name__ == '__main__':")?;
    writeln!(main_py, "    hello()")?;

    // Create the `tests` folder with `__init__.py` and `test_main.py`
    let tests_dir = base_path.join("tests");
    fs::create_dir_all(&tests_dir)?;
    File::create(tests_dir.join("__init__.py"))?;
    let mut test_main_py = File::create(tests_dir.join("test_greet.py"))?;
    writeln!(test_main_py, "from {source_name}.greet import hello")?;
    writeln!(test_main_py, "\n\ndef test_hello():")?;
    writeln!(
        test_main_py,
        "    hello(\"test\")  # Should print 'Hello, test!'"
    )?;

    // Create an improved pyproject.toml
    let mut pyproject = File::create(base_path.join("pyproject.toml"))?;
    writeln!(
        pyproject,
        r#"[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[project]
name = "{project_name}"
version = "0.1.0"
description = "A Python project named {project_name}"
authors = []
requires-python = ">=3.9.0"
dependencies = []
readme = {{file = "README.md", content-type = "text/markdown"}}

[project.optional-dependencies]
dev = [
    "ruff",
    "pytest",
    "twine", 
    "maturin",
]

[tool.setuptools]
packages = {{find = {{}}}}
include-package-data = true
"#
    )?;

    // Create README.md
    let mut readme = File::create(base_path.join("README.md"))?;
    writeln!(readme, "# {}\n\nA new Python project.\n\n", project_name)?;
    writeln!(
        readme,
        "## Installation\n\n```bash\nsource .{}/bin/activate\nuv pip install \".[dev]\"\n```\n",
        project_name
    )?;

    // Check if Git is installed, then initialize a Git repository
    if is_git_installed() {
        Command::new("git")
            .arg("init")
            .current_dir(base_path)
            .output()
            .expect("Failed to initialize git repository");

        // Create .gitignore file
        let mut gitignore = File::create(base_path.join(".gitignore"))?;
        writeln!(gitignore, "# Byte-compiled / optimized / DLL files")?;
        writeln!(gitignore, "__pycache__/")?;
        writeln!(gitignore, "*.py[cod]")?;
        writeln!(gitignore, "*$py.class")?;
        writeln!(gitignore, ".pytest_cache")?;
        writeln!(gitignore, "")?;

        writeln!(gitignore, "# Distribution / packaging")?;
        writeln!(gitignore, "build/")?;
        writeln!(gitignore, "dist/")?;
        writeln!(gitignore, "*.egg-info/")?;
        writeln!(gitignore, "*.egg")?;
        writeln!(gitignore, "")?;

        writeln!(gitignore, "# Environments")?;
        writeln!(gitignore, ".env")?;
        writeln!(gitignore, ".venv")?;
        writeln!(gitignore, "env/")?;
        writeln!(gitignore, "venv/")?;
        writeln!(gitignore, "")?;

        writeln!(gitignore, "# Jupyter Notebook")?;
        writeln!(gitignore, ".ipynb_checkpoints")?;
        writeln!(gitignore, "")?;

        writeln!(gitignore, "# macOS")?;
        writeln!(gitignore, ".DS_Store")?;
        writeln!(gitignore, "")?;

        writeln!(gitignore, "# Editor directories and files")?;
        writeln!(gitignore, ".idea/")?;
        writeln!(gitignore, ".vscode/")?;
        writeln!(gitignore, "*.swp")?;
        writeln!(gitignore, "*.swo")?;

        println!("Initialized Git repository and created .gitignore.");
    } else {
        println!("Warning: Git is not installed. Skipping Git repository initialization.");
    }

    if is_uv_installed() {
        println!("Detected `uv`. Creating virtual environment...");
        let uv_command = Command::new("uv")
            .args(&["venv", ".venv"])
            .current_dir(base_path)
            .output()
            .expect("Failed to create virtual environment using `uv`.");

        if !uv_command.status.success() {
            eprintln!("Error: Failed to create virtual environment using `uv`.");
        } else {
            println!("Virtual environment `.venv` created successfully.\n");
        }

        // Check if the virtual environment exists
        if venv_path.exists() {
            println!("To activate the virtual environment, run:");
            println!("    source .venv/bin/activate");
            println!(
                "\nThis will activate the virtual environment for project '{}'.",
                project_name
            );
        } else {
            eprintln!(
                "Error: Virtual environment for project '{}' not found.",
                project_name
            );
        }

        // Install the current packages in the virtual environment
        let uv_pip_install = Command::new("uv")
            .args(&["pip", "install", "-e", ".[dev]"])
            .current_dir(base_path)
            .output()
            .expect("Failed to install the project in the virtual environment.");

        if !uv_pip_install.status.success() {
            eprintln!("Error: Failed to install the project in the virtual environment.");
        } else {
            println!("Project installed successfully in the virtual environment.\n");
        }
    } else {
        println!("`uv` is not installed! Virtual environment is not created.\n");
        println!("Follow the instructions at https://docs.astral.sh/uv/#getting-started to install `uv`.");
    }

    println!("\nProject '{project_name}' created successfully!");
    Ok(())
}
