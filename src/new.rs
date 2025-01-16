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

    // Create the `project_name` directory inside the base path with `__init__.py` and `main.py`
    let project_dir = base_path.join(project_name);
    fs::create_dir_all(&project_dir)?;

    // Create `__init__.py` (empty)
    File::create(project_dir.join("__init__.py"))?;

    // Create `main.py` with a "Hello, world!" example
    let mut main_py = File::create(project_dir.join("main.py"))?;
    writeln!(main_py, "def main():")?;
    writeln!(main_py, "    print('Hello, world!')")?;
    writeln!(main_py, "\nif __name__ == '__main__':")?;
    writeln!(main_py, "    main()")?;

    // Create the `tests` folder with `__init__.py` and `test_main.py`
    let tests_dir = base_path.join("tests");
    fs::create_dir_all(&tests_dir)?;
    File::create(tests_dir.join("__init__.py"))?;
    let mut test_main_py = File::create(tests_dir.join("test_main.py"))?;
    writeln!(test_main_py, "from {}.main import main", project_name)?;
    writeln!(test_main_py, "def test_main():")?;
    writeln!(test_main_py, "    # Ideally use a proper assert")?;
    writeln!(test_main_py, "    main()  # Should print 'Hello, world!'")?;

    // Create an improved pyproject.toml
    let mut pyproject = File::create(base_path.join("pyproject.toml"))?;
    writeln!(
        pyproject,
        r#"[build-system]
requires = ["setuptools>=42", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "{project_name}"
version = "0.1.0"
description = "A Python project named {project_name}"
authors = []
requires-python = ">=3.9.0"
dependencies = []

[project.optional-dependencies]
dev = [
    "black",
    "isort",
    "pytest",
]

[tool.setuptools]
packages = {{find = {{}}}}
include-package-data = true
"#
    )?;

    // Create README.md
    let mut readme = File::create(base_path.join("README.md"))?;
    writeln!(readme, "# {}\n\nA new Python project.", project_name)?;

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

    println!("Project '{}' created successfully!", project_name);
    Ok(())
}

/// Helper function to check if Git is installed
fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
