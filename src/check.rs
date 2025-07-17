use anyhow::{Result, bail};
use semver::Version;

use crate::structs::PackConfig;

pub fn check_semver(config: &PackConfig) -> bool {
    Version::parse(&config.version).is_ok()
}

pub fn assert_valid_config(config: &PackConfig) -> Result<()> {
    if !check_semver(config) {
        bail!("invalid version, version must be valid semver")
    }

    Ok(())
}
