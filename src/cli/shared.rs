use anyhow::{bail, Context, Result};
use std::{fs, path::PathBuf};

#[derive(serde::Deserialize)]
pub struct RepositoryCargo {
    pub workspace: WorkspaceCargo,
}

#[derive(serde::Deserialize)]
pub struct WorkspaceCargo {
    pub metadata: WorkspaceMetadataCargo,
    pub dependencies: WorkspaceDependenciesCargo,
}

#[derive(serde::Deserialize)]
pub struct WorkspaceDependenciesCargo {
    pub mochi: MochiDependencyCargo,
}

#[derive(serde::Deserialize)]
pub struct MochiDependencyCargo {
    pub version: String,
}

#[derive(serde::Deserialize)]
pub struct WorkspaceMetadataCargo {
    pub mochi: RepositoryManifest,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RepositoryManifest {
    pub name: String,
    pub author: String,
    pub description: Option<String>,
}

pub fn validate_workspace(path: Option<PathBuf>) -> Result<(PathBuf, RepositoryCargo)> {
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
