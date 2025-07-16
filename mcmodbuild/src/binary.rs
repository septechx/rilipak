use crate::structs::{BuildType, ModBuild};
use anyhow::{Ok, Result};
use oxfmt::Deserialize;

pub fn deserialize(buf: &[u8]) -> Result<ModBuild> {
    let header = "mcmodbuild".as_bytes();
    let version: u16 = 1;

    let mut deserialize = Deserialize::new(buf).init(header, version)?;

    let id = deserialize.read_string()?;
    let name = deserialize.read_string()?;
    let git = deserialize.read_string()?;
    let branch = deserialize.read_string()?;
    let build_int = deserialize.read_u8()?;
    let build = BuildType::try_from(build_int)?;
    let cmd = match build {
        BuildType::Cmd => Some(deserialize.read_string()?),
        BuildType::Std => None,
    };
    let out = deserialize.read_string()?;
    let exclude = deserialize.read_vec()?;

    Ok(ModBuild {
        id,
        name,
        git,
        branch,
        build,
        cmd,
        out,
        exclude,
    })
}
