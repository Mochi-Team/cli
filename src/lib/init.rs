use clap::Subcommand;

#[derive(Subcommand)]
pub enum InitCmd {
    /// Create a module
    Module
    // TODO: Add repository builder
    // Repository
}

pub fn handle(cmd: InitCmd) {
    match cmd {
        // Initialize module
        InitCmd::Module => {
            init_module();
        }
    }
}

pub fn init_module() {
    println!("Create new source")
}