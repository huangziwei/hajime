mod build;
mod new;
mod publish;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hajime")]
#[command(about = "A Rust CLI for Python project management", long_about = None, version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

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
    /// Log in to PyPI by storing the token securely
    Publish {
        /// PyPI account to use (default if not specified)
        #[arg(short, long, help = "PyPI account to use for publishing")]
        account: Option<String>,

        /// Override the token for the specified account
        #[arg(
            short = 'o',
            long = "override-token",
            help = "Override the token for the specified account"
        )]
        override_token: bool,
    },
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
        Commands::Publish {
            account,
            override_token,
        } => {
            if let Err(e) = publish::publish_package(account.clone(), *override_token) {
                eprintln!("Error publishing package: {}", e);
            }
        }
    }
}
