use anyhow::Result;
use clap::{Parser, Subcommand};

mod build;
mod init;
mod serve;
mod shared;

#[derive(Parser)]
#[clap(about, author, version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Initializes a repository or module.
    #[command(subcommand)]
    Init(init::InitCmd),

    /// Builds repository from modules.
    Build(build::BuildCmd),

    /// Builds repository and starts a local server.
    Serve(serve::WebserverCmd),
}

impl Cli {
    pub fn handle() -> Result<()> {
        let cli = Cli::parse();

        match cli.command {
            Cmd::Init(cmd) => init::handle(cmd),
            Cmd::Build(cmd) => build::handle(cmd),
            Cmd::Serve(cmd) => serve::handle(cmd),
        }
    }
}
