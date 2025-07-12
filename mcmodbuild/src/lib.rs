mod binary;
mod installer;
mod structs;
use crate::binary::deserialize;
use crate::installer::Installer;
use anyhow::Result;
use std::path::PathBuf;

/// Builds a mod from a ModBuild binary and returns the path to the built file in the cache directory.
pub fn build(modbuild: &[u8]) -> Result<PathBuf> {
    let build = deserialize(modbuild)?;
    let installer = Installer::new(build)?;
    installer.ensure_cache_directory()?;
    installer.clone_or_update_repository()?;
    installer.build_project()?;
    let output_path = installer.get_built_files()?;
    Ok(output_path)
}
