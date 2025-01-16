# Hajime (始め)

**hajime** is a lightweight Rust CLI tool designed to quickly create and manage Python project skeletons. 

## Installation

To install hajime locally, clone this repository and run:

```bash
cargo install --path .
```

Make sure `~/.cargo/bin` is in your `PATH`.

## Usage

### Create a New Python Project
Run the following command to create a new Python project:

```bash
hajime new <project_name>
```

Example:
```bash
hajime new my_project
```

This creates the following structure:
```
my_project/
├── README.md
├── pyproject.toml
├── my_project
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

This runs `python3 -m build` to generate a distributable wheel.
