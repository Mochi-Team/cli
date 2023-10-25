use clap::Subcommand;
use std::{fs, process::Command, path::Path};
use tera::{Tera, Context};

#[derive(Subcommand)]
pub enum CompileCmd {
    Repository
}

#[derive(serde::Deserialize)]
struct RepositoryCargo {
    workspace: WorkspaceCargo
}

#[derive(serde::Deserialize)]
struct WorkspaceCargo {
    metadata: WorkspaceMetadataCargo
}

#[derive(serde::Deserialize)]
struct WorkspaceMetadataCargo {
    mochi: RepositoryManifest
}

#[derive(serde::Deserialize)]
struct ModuleCargo {
    package: ModulePackageCargo,
}

#[derive(serde::Deserialize)]
struct ModulePackageCargo {
    metadata: MetadataCargo,
    name: String,
    version: String
}

#[derive(serde::Deserialize)]
struct MetadataCargo {
    mochi: MochiCargo
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
    mochi_version: String
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

pub fn handle(cmd: CompileCmd) {
    match cmd {
        CompileCmd::Repository => compile_repository()
    }
}

fn compile_repository() {
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .status()
        .expect("failed to build modules");

    if !status.success() {
        println!("There was an issue compiling modules. {}", status)
    }

    let cwd = std::env::current_dir().expect("failed to get current working directory");
    let target_releases_path = cwd
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release");

    let repo_cargo_path = cwd.join("Cargo").with_extension("toml");
    let repo_manifest = toml::from_str::<RepositoryCargo>(
        &fs::read_to_string(repo_cargo_path).expect("No `Cargo.toml` found in directory for repository."),
    )
    .unwrap()
    .workspace
    .metadata
    .mochi;

    // Iterate every module and store in dist directory.

    let dist_path = cwd.join("dist");
    let dist_modules_path = dist_path.join("modules");

    // delete dist if present
    _ = fs::remove_dir_all(&dist_path);

    fs::create_dir_all(&dist_modules_path).expect("failed to create `dist/modules` folder.");

    let modules_dir = cwd.join("modules");

    let normalized_author = normalize_string(&repo_manifest.author).to_lowercase();
    assert!(normalized_author.len() > 0);

    let mut releases = RepositoryReleaseManifest {
        modules: vec![],
        repository: repo_manifest,
    };

    for entry in fs::read_dir(modules_dir).expect("failed to retrieve modules") {
        let module_cargo_path = entry
            .expect("failed to retrieve module")
            .path()
            .join("Cargo")
            .with_extension("toml");

        if !module_cargo_path.exists() {
            continue;
        }

        let module_cargo_str =
            fs::read_to_string(&module_cargo_path).expect("failed to retrieve module's cargo");
        let module_cargo: ModuleCargo =
            toml::from_str(&module_cargo_str).expect("failed to unpack module");

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
        .expect(
            format!(
                "failed to copy wasm file to dist for {}",
                &module_cargo.package.name
            )
            .as_str(),
        );

        let module_manifest = ModuleManifest {
            id: module_id.clone(),
            name: module_cargo.package.metadata.mochi.name,
            description: module_cargo.package.metadata.mochi.description.map(|f| f.trim().into()),
            file: format!("/modules/{}.wasm", &module_id),
            version: module_cargo.package.version,
            meta: vec![],
            icon: module_cargo.package.metadata.mochi.icon,
            // TODO: set correct mochi bindings version
            mochi_version: "0.0.2".into()
        };
        releases.modules.push(module_manifest);
    }

    geerate_html_template(&releases, &dist_path);

    fs::write(
        dist_path.join("Manifest").with_extension("json"),
        serde_json::to_string_pretty(&releases).expect("failed to create `Manifest.json`"),
    )
    .unwrap();

    println!("Successfully packaged server!")
}

fn geerate_html_template(manifest: &RepositoryReleaseManifest, output_path: &Path) {
    let index_bytes = include_str!("../../templates/site/index.html");

    let mut tera = Tera::default();
    tera.add_raw_template("index.html", index_bytes)
    .expect("failed to create index.html template.");

    let mut context = Context::new();
    context.insert("repository", &manifest.repository);
    context.insert("modules", &manifest.modules);
    let rendered = tera.render("index.html", &context).expect("failed to create template for index.html");

    fs::write(
        output_path.join("index").with_extension("html"),
        rendered,
    )
    .unwrap();
}

fn normalize_string(value: &String) -> String {
    return value
        .trim()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .replace(" ", "-");
}