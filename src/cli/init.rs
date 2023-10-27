use std::path::{Path, PathBuf};

use anyhow::{Context, Ok, Result};
use convert_case::{Case, Casing};
use tera::Tera;

#[derive(clap::Subcommand)]
pub enum InitCmd {
    /// Creates a module from a template.
    Module(ModuleArgs),

    /// Creates a repository from a template.
    Repository(RepositoryArgs),
}

#[derive(serde::Serialize, clap::Parser)]
pub struct ModuleArgs {
    /// Display name for the module.
    ///
    /// Must not be empty and must be unique across all your other modules.
    #[arg(short, long)]
    pub name: String,
}

#[derive(serde::Serialize, clap::Parser)]
pub struct RepositoryArgs {
    /// Display name for the repository.
    ///
    /// Must not be empty.
    #[arg(long)]
    name: String,

    /// Author of the repository.
    ///
    /// Git username is highly recommended. Must not be empty and must be unique.
    #[arg(long)]
    author: String,

    /// Output path to the repository. By default it uses the cwd + `/repository-name/`.
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(serde::Serialize)]
struct ModuleTemplate {
    display_name: String,
    struct_name: String,
    identifier_name: String,
}

pub fn handle(cmd: InitCmd) -> Result<()> {
    match cmd {
        InitCmd::Module(args) => init_module(args),
        InitCmd::Repository(args) => init_repository(args),
    }
}

fn init_repository(args: RepositoryArgs) -> Result<()> {
    let output_path = args.output.clone().unwrap_or(
        std::env::current_dir()
            .with_context(|| "failed to get current working directory")?
            .join(args.name.to_case(Case::Kebab)),
    );

    _ = std::fs::create_dir_all(&output_path);

    let mut context = tera::Context::new();
    context.insert("repository", &args);

    let repository_template_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates/repository");

    recursive_copy_template(&output_path, &context, &repository_template_dir, None)?;

    println!("Successfully generated {} repository!", args.name);

    Ok(())
}

fn init_module(args: ModuleArgs) -> Result<()> {
    let (workspace_dir, _) = super::shared::validate_workspace(None)?;

    let module_snake_case = args.name.to_case(Case::Snake);

    let module_template = ModuleTemplate {
        struct_name: args.name.to_case(Case::Pascal),
        identifier_name: args.name.to_case(Case::Kebab),
        display_name: args.name,
    };

    let mut context = tera::Context::new();
    context.insert("module", &module_template);

    let module_dir = workspace_dir.join("modules").join(module_snake_case);

    _ = std::fs::create_dir(&module_dir);

    let module_template_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates/module");

    recursive_copy_template(&module_dir, &context, &module_template_dir, None)?;

    println!(
        "Successfully generated {} module!",
        module_template.display_name
    );

    Ok(())
}

fn recursive_copy_template(
    output_dir: &Path,
    tera_context: &tera::Context,
    base_template_path: &PathBuf,
    path: Option<&PathBuf>,
) -> Result<()> {
    let path = path.unwrap_or(base_template_path);

    // println!("Search path: {}", path.display());

    if path.is_file() {
        let file_path = path;
        let is_template = file_path.extension().is_some_and(|e| e == "tpl");
        let stripped_file_path = file_path.strip_prefix(base_template_path)?;
        // println!("  file(template = {}): {}", is_template, &file_path.display());

        if is_template {
            let mut tera = Tera::default();
            let file_bytes = std::fs::read_to_string(file_path)
                .with_context(|| "failed to convert template to bytes")?;
            tera.add_raw_template("template", &file_bytes)
                .with_context(|| "failed to add template to engine")?;
            tera.build_inheritance_chains()
                .with_context(|| "failed to build template")?;

            let rendered = tera
                .render("template", tera_context)
                .with_context(|| "failed to render template")?;
            std::fs::write(
                output_dir.join(stripped_file_path.with_extension("")),
                rendered,
            )
            .with_context(|| "failed to copy rendered template to file")?;
        } else {
            std::fs::copy(file_path, output_dir.join(stripped_file_path))?;
        }
    } else {
        for entry in std::fs::read_dir(path).with_context(|| "failed to read path dir")? {
            let entry = entry.with_context(|| "entry is nil")?;
            let entry_path = entry.path();
            let stripped_prefix_entry = entry_path.strip_prefix(base_template_path)?;

            // println!("  entry: {}", &stripped_prefix_entry.display());

            if entry_path.is_dir() {
                std::fs::create_dir(output_dir.join(stripped_prefix_entry))?;
            }

            recursive_copy_template(
                output_dir,
                tera_context,
                base_template_path,
                Some(&entry_path),
            )?;
        }
    }
    Ok(())
}
