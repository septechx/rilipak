use anyhow::{Result, bail};
use oxfmt::{BinaryBuilder, Serializable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Serializable)]
pub struct PackConfig {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub mods: Vec<Mod>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Serializable)]
pub struct Mod {
    pub source: ModSource,
    pub id: String,
    pub env: ModEnv,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Serializable)]
#[repr(u8)]
pub enum ModSource {
    Curseforge = 0,
    Modrinth = 1,
    Github = 2,
}

impl TryFrom<u8> for ModSource {
    type Error = anyhow::Error;
    fn try_from(v: u8) -> Result<Self> {
        match v {
            0 => Ok(ModSource::Curseforge),
            1 => Ok(ModSource::Modrinth),
            2 => Ok(ModSource::Github),
            other => bail!("invalid mod source: {}", other),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Serializable)]
#[repr(u8)]
pub enum ModEnv {
    Server = 0,
    Client = 1,
    Common = 2,
}

impl TryFrom<u8> for ModEnv {
    type Error = anyhow::Error;
    fn try_from(v: u8) -> Result<Self> {
        match v {
            0 => Ok(ModEnv::Server),
            1 => Ok(ModEnv::Client),
            2 => Ok(ModEnv::Common),
            other => bail!("invalid mod source: {}", other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serializable)]
#[oxfmt(header = "rilipak", version = 1)]
pub struct Pack {
    pub meta: PackMeta,
    pub include: Box<[u8]>,
}

#[derive(Debug, Clone, PartialEq, Serializable)]
pub struct PackMeta {
    pub config: PackConfig,
    pub modbuilds: Vec<Box<[u8]>>,
}
