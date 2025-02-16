mod build;
mod check;
pub mod helpers;
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
    Build {
        /// Use maturin to build the project
        #[arg(short = 'm', long, help = "Use maturin to build the project")]
        maturin: bool,

        /// Bump the project version (macro, meso, micro)
        #[arg(
            short = 'b',
            long,
            help = "Bump the project version (macro, meso, micro)",
            value_parser = ["macro", "meso", "micro"]
        )]
        bump_version: Option<String>,
    },
    /// Check th build of the Python project
    Check,
    /// Publish the project to PyPI
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

        /// Use maturin for uploading Rust-based Python projects
        #[arg(long, help = "Use maturin for uploading Rust-based Python projects")]
        maturin: bool,
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
        Commands::Build {
            maturin,
            bump_version,
        } => {
            build::build_project(*maturin, bump_version.as_deref());
        }
        Commands::Check => {
            if let Err(e) = check::check_package() {
                eprintln!("Error checking package: {}", e);
            }
        }
        Commands::Publish {
            account,
            override_token,
            maturin,
        } => {
            if let Err(e) = publish::publish_package(account.clone(), *override_token, *maturin) {
                eprintln!("Error publishing package: {}", e);
            }
        }
    }
}
