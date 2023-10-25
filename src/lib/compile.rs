use anyhow::{bail, Context as AnyContext, Result};
use clap::Subcommand;
use std::{fs, path::Path, process::Command};
use tera::{Context, Tera};

#[derive(Subcommand)]
pub enum CompileCmd {
    Repository,
}

#[derive(serde::Deserialize)]
struct RepositoryCargo {
    workspace: WorkspaceCargo,
}

#[derive(serde::Deserialize)]
struct WorkspaceCargo {
    metadata: WorkspaceMetadataCargo,
}

#[derive(serde::Deserialize)]
struct WorkspaceMetadataCargo {
    mochi: RepositoryManifest,
}

#[derive(serde::Deserialize)]
struct ModuleCargo {
    package: ModulePackageCargo,
}

#[derive(serde::Deserialize)]
struct ModulePackageCargo {
    metadata: MetadataCargo,
    name: String,
    version: String,
}

#[derive(serde::Deserialize)]
struct MetadataCargo {
    mochi: MochiCargo,
}

#[derive(serde::Deserialize)]
struct MochiCargo {
    name: String,
    description: Option<String>,
    icon: Option<String>,
}

// JSON Serialization

#[derive(serde::Deserialize, serde::Serialize)]
struct RepositoryManifest {
    name: String,
    author: String,
    description: Option<String>,
}

#[derive(serde::Serialize)]
struct ModuleManifest {
    id: String,
    name: String,
    description: Option<String>,
    file: String,
    version: String,
    meta: Vec<MetaType>,
    icon: Option<String>,
    mochi_version: String,
}

#[derive(serde::Serialize)]
struct RepositoryReleaseManifest {
    repository: RepositoryManifest,
    modules: Vec<ModuleManifest>,
}

#[derive(serde::Serialize)]
#[allow(dead_code)]
enum MetaType {
    Video,
    Image,
    Text,
}

pub fn handle(cmd: CompileCmd) -> Result<()> {
    match cmd {
        CompileCmd::Repository => compile_repository(),
    }
}

fn compile_repository() -> Result<()> {
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .status()
        .with_context(|| "Failed to build modules")?;

    if !status.success() {
        bail!(
            "There was an issue building modules. Rust Cargo err: {}",
            status
        );
    }

    let cwd = std::env::current_dir().with_context(|| "failed to get current working directory")?;

    let target_releases_path = cwd
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release");

    let repo_cargo_path = cwd.join("Cargo").with_extension("toml");
    let repo_manifest = toml::from_str::<RepositoryCargo>(
        &fs::read_to_string(repo_cargo_path)
            .with_context(|| "No `Cargo.toml` found in directory for repository.")?,
    )
    .with_context(|| "Failed to deserialize Repository's `Cargo.toml` metadata info.")?
    .workspace
    .metadata
    .mochi;

    // Iterate every module and store in dist directory.

    let dist_path = cwd.join("dist");
    let dist_modules_path = dist_path.join("modules");

    // delete dist if present
    _ = fs::remove_dir_all(&dist_path);

    fs::create_dir_all(&dist_modules_path)
        .with_context(|| "Failed to create `dist/modules` folder.")?;

    let modules_dir = cwd.join("modules");

    let normalized_author = normalize_string(&repo_manifest.author).to_lowercase();

    if normalized_author.is_empty() {
        bail!("Author's name in Repository's `Cargo.toml` must follow Unicode XID.")
    }

    let mut releases = RepositoryReleaseManifest {
        modules: vec![],
        repository: repo_manifest,
    };

    for entry in
        fs::read_dir(modules_dir).with_context(|| "Failed to retrieve modules directories.")?
    {
        let module_dir = entry
            .context("Failed to retrieve module directory ")?
            .path();

        let module_cargo_path = module_dir.join("Cargo").with_extension("toml");

        if !module_cargo_path.exists() {
            continue;
        }

        let module_cargo_str = fs::read_to_string(&module_cargo_path).with_context(|| {
            format!(
                "Failed to retrieve module's `Cargo.toml` for {}",
                &module_dir.display()
            )
        })?;

        let module_cargo: ModuleCargo = toml::from_str(&module_cargo_str).with_context(|| {
            format!(
                "Failed to parse module's `Cargo.toml` for {}",
                &module_dir.display()
            )
        })?;

        let module_id = format!(
            "com.{}.{}",
            normalized_author,
            module_cargo.package.name.to_lowercase()
        );

        // TODO: Zip Modules with their resources

        fs::copy(
            target_releases_path
                .join(&module_cargo.package.name)
                .with_extension("wasm"),
            dist_modules_path
                .join(format!("{}.stub", &module_id))
                .with_extension("wasm"),
        )
        .with_context(|| {
            format!(
                "Failed to copy wasm file to dist for {}",
                &module_cargo.package.name
            )
        })?;

        let module_manifest = ModuleManifest {
            id: module_id.clone(),
            name: module_cargo.package.metadata.mochi.name,
            description: module_cargo
                .package
                .metadata
                .mochi
                .description
                .map(|f| f.trim().into()),
            file: format!("/modules/{}.wasm", &module_id),
            version: module_cargo.package.version,
            meta: vec![],
            icon: module_cargo.package.metadata.mochi.icon,
            // TODO: set correct mochi bindings version
            mochi_version: "0.0.2".into(), //TODO: add md5 for "security"
        };
        releases.modules.push(module_manifest);
    }

    geerate_html_template(&releases, &dist_path)?;

    fs::write(
        dist_path.join("Manifest").with_extension("json"),
        serde_json::to_string_pretty(&releases)
            .with_context(|| "Failed to serialize to `Manifest.json` for Repository")?,
    )
    .with_context(|| "There was an issue writing to `Manifest.json` for Repository")?;

    println!("Successfully packaged server!");
    Ok(())
}

fn geerate_html_template(manifest: &RepositoryReleaseManifest, output_path: &Path) -> Result<()> {
    let index_bytes = include_str!("../../templates/site/index.html");

    let mut tera = Tera::default();
    tera.add_raw_template("index.html", index_bytes)
        .with_context(|| "Failed to find `index.html` template.")?;

    let mut context = Context::new();
    context.insert("repository", &manifest.repository);
    context.insert("modules", &manifest.modules);
    let rendered = tera
        .render("index.html", &context)
        .with_context(|| "Failed to render `index.html` template.")?;

    fs::write(output_path.join("index").with_extension("html"), rendered)
        .with_context(|| "Failed to write `index.html` to dist/")
}

fn normalize_string(value: &str) -> String {
    return value
        .trim()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .replace(' ', "-");
}
