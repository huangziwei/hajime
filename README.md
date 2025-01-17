# Hajime (始め)

**hajime** is a lightweight Rust CLI tool designed to quickly create and manage Python project skeletons. 

## Installation

```bash
cargo install hajime
```

Or, to install hajime locally, clone this repository and run:

```bash
cargo install --path .
```

Make sure `~/.cargo/bin` is in your `PATH`.

## Usage

### Create a New Python Project
Run the following command to create a new Python project:

```bash
hajime new project_name
```

This creates the following structure:
```
project_name/
├── README.md
├── pyproject.toml
├── project_name
│   ├── __init__.py
│   └── main.py
├── tests
│   ├── __init__.py
│   └── test_main.py
```

### Build the Project
To build the Python project into a wheel:

```bash
hajime build
```

This runs `python3 -m build` and will package your project and place the distribution files (e.g., `.tar.gz` and `.whl`) in the `dist/` directory.

### Check the build

To check the build, run:

```bash
hajime check
```

### Publish the Project to PyPI
To publish your project to [PyPI](https://pypi.org), run the following command:

```bash
hajime publish
```

By default, `hajime` will use the `default` account stored in your system's keyring. You can specify an account or override the stored token:

#### Specify an Account

```bash
hajime publish --account account_name
```

#### Override the Token for an Account

```bash
hajime publish --account account_name --override-token
```

This will:
1. Prompt you to enter a new token.
2. Save the token securely in the keyring.
3. Publish the package to PyPI.

---

#### Notes
- Ensure you have built the project first using `hajime build` or `python3 -m build`.
- If the required `build` or `twine` packages are missing, install them with:
  ```bash
  pip install build twine
  ```
- For accounts, `hajime` securely stores your PyPI API tokens in your system's keyring (e.g., macOS Keychain, Windows Credential Manager, or Linux Secret Service).