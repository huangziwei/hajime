mod build;
mod new;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hajime")]
#[command(about = "A Rust CLI for Python project management", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Python project
    New {
        #[arg(help = "Name of the new project")]
        project_name: String,

        /// Overwrite the existing project if it already exists
        #[arg(
            short,
            long,
            help = "Force project creation, overwriting existing files"
        )]
        force: bool,
    },
    /// Build the Python project
    Build,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New {
            project_name,
            force,
        } => {
            if let Err(e) = new::create_project(project_name, *force) {
                eprintln!("Error creating project: {}", e);
            }
        }
        Commands::Build => build::build_project(),
    }
}
