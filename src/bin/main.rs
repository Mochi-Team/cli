use clap::{Parser, Subcommand};
use api;

#[derive(Parser)]
#[clap(
    about = "A CLI tool for managing mochi-based repository and modules",
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
    // TODO: add init
    // #[command(subcommand)]
    // Init(api::init::InitCmd),

    /// Compile a repository or a module for mochi.
    #[command(subcommand)]
    Compile(api::compile::CompileCmd),

    /// Start a webserver to test your repo and modules.
    #[command(subcommand)]
    Webserver(api::webserver::WebserverCmd)
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        // Cmd::Init(cmd) => api::init::handle(cmd),
        Cmd::Webserver(cmd) => api::webserver::handle(cmd),
        Cmd::Compile(cmd) => api::compile::handle(cmd)
    }
}