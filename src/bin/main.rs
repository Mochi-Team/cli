use clap::{Parser, Subcommand};
use api;

#[derive(Parser)]
#[clap(
    about = "A CLI tool for managing mochi-based repository and sources",
    version,
    author
)]
struct Cli {
    #[command(subcommand)]
    command: Cmd
}

#[derive(Subcommand)]
enum Cmd {
    /// Initialize a new repository or source module.
    #[command(subcommand)]
    Init(InitCmd)
}

#[derive(Subcommand)]
enum InitCmd {
    /// Create a source module
    Source
    // TODO: Add repository template builder support
    // Repository
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        // Initialize
        Cmd::Init(init_type) => {
            match init_type {
                // Initialize source
                InitCmd::Source => {
                    api::init::init_source();
                }
            }
        }
    }
}