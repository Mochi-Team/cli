include!("src/lib.rs");

use std::path::Path;

fn main() -> std::io::Result<()> {
    let dest_path = Path::new("./").join("CommandLineHelp.md");

    std::fs::write(dest_path, clap_markdown::help_markdown::<Cli>())?;

    Ok(())
}
