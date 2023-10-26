use anyhow::{bail, Result};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum InitCmd {
    Module,
    Repository,
}

pub fn handle(cmd: InitCmd) -> Result<()> {
    match cmd {
        InitCmd::Module => init_module(),
        InitCmd::Repository => init_repository(),
    }
}

fn init_repository() -> Result<()> {
    bail!("Init Cmd not implemented for repository")
}

fn init_module() -> Result<()> {
    bail!("Init Cmd not implemented for module")
}
