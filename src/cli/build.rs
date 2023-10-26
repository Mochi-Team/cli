use anyhow::{bail, Context as AnyContext, Result};
use clap::Parser;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use tera::{Context, Tera};

pub type BuildCmd = BuildArgs;

#[derive(Parser, Default)]
pub struct BuildArgs {
    /// Path of workspace.
    /// (Defaults to current working directory)
    #[arg(long)]
    pub path: Option<PathBuf>,

    /// Output path for generated repository. For more info use `--help`
    ///
    /// Defaults to `path` + `/dist/` dir, if `path is specified, or current working directory + `/dist/
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Include generated static site for repository
    ///
    /// This creates an `index.html` on the `output` path or cwd + `/dist/` if argument is present.
    #[arg(short, long, default_value_t = false)]
    pub site: bool,
}

#[derive(serde::Deserialize)]
struct RepositoryCargo {
    workspace: WorkspaceCargo,
}

#[derive(serde::Deserialize)]
struct WorkspaceCargo {
    metadata: WorkspaceMetadataCargo,
    dependencies: WorkspaceDependenciesCargo,
}

#[derive(serde::Deserialize)]
struct WorkspaceDependenciesCargo {
    mochi: MochiDependencyCargo,
}

#[derive(serde::Deserialize)]
struct MochiDependencyCargo {
    version: String,
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
    hash_value: String,
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

pub fn handle(cmd: BuildCmd) -> Result<()> {
    compile_repository(cmd)
}

fn compile_repository(args: BuildArgs) -> Result<()> {
    let (workspace_dir, workspace_cargo) = validate_workspace(args.path)?;

    execute_builds(&workspace_dir)?;

    let repository_manifest = workspace_cargo.workspace.metadata.mochi;

    let dist_path = args.output.unwrap_or(workspace_dir.join("dist"));

    let dist_modules_path = dist_path.join("modules");

    _ = fs::remove_dir_all(&dist_modules_path);

    fs::create_dir_all(&dist_modules_path).with_context(|| "Failed to create `output` folder.")?;

    let modules_dir = workspace_dir.join("modules");
    let normalized_author = normalize_string(&repository_manifest.author).to_lowercase();

    if normalized_author.is_empty() {
        bail!("Author's name in Repository's `Cargo.toml` must follow Unicode XID.")
    }

    let mut release = RepositoryReleaseManifest {
        modules: vec![],
        repository: repository_manifest,
    };

    let target_releases_path = workspace_dir
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release");

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
        // TODO: Reduce the file size of wasm.

        let wasm_destination_path = dist_modules_path.join(format!("{}.wasm", &module_id));

        fs::copy(
            target_releases_path.join(format!("{}.wasm", &module_cargo.package.name)),
            &wasm_destination_path,
        )
        .with_context(|| {
            format!(
                "Failed to copy wasm file to dist for {}",
                &module_cargo.package.name
            )
        })?;

        let hashed_file_value = sha256::try_digest(wasm_destination_path).with_context(|| {
            format!(
                "Failed to generate hash value for {}'s file",
                &module_cargo.package.metadata.mochi.name
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
            mochi_version: workspace_cargo.workspace.dependencies.mochi.version.clone(),
            hash_value: hashed_file_value,
        };
        release.modules.push(module_manifest);
    }

    if args.site {
        generate_html_template(&release, &dist_path)?;
    }

    fs::write(
        dist_path.join("Manifest").with_extension("json"),
        serde_json::to_string_pretty(&release)
            .with_context(|| "Failed to serialize to `Manifest.json` for Repository")?,
    )
    .with_context(|| "There was an issue writing to `Manifest.json` for Repository")?;

    println!("Successfully packaged server!");
    Ok(())
}

fn validate_workspace(path: Option<PathBuf>) -> Result<(PathBuf, RepositoryCargo)> {
    let workspace_directory = path.unwrap_or(
        std::env::current_dir().with_context(|| "failed to get current working directory")?,
    );

    if !workspace_directory.is_dir() {
        bail!(format!(
            "{} is not a valid directory.",
            workspace_directory.to_str().unwrap_or_default()
        ));
    }

    let repo_cargo_path = workspace_directory.join("Cargo").with_extension("toml");
    toml::from_str::<RepositoryCargo>(
        &fs::read_to_string(repo_cargo_path)
            .with_context(|| "No `Cargo.toml` found in repository's workspace.")?,
    )
    .with_context(|| "Failed to deserialize Repository's `Cargo.toml` metadata info.")
    .map(|v| (workspace_directory, v))
}

fn execute_builds(dir: &Path) -> Result<()> {
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--manifest-path")
        .arg(format!("{}/Cargo.toml", dir.display()))
        .status()
        .with_context(|| "Failed to build modules.")?;

    if !status.success() {
        bail!(
            "There was an issue building modules. Rust Cargo err: {}",
            status
        );
    }

    Ok(())
}

fn generate_html_template(manifest: &RepositoryReleaseManifest, output_path: &Path) -> Result<()> {
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
