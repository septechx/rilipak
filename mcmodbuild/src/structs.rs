use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BuildType {
    Cmd = 0,
    Std = 1,
}

impl TryFrom<u8> for BuildType {
    type Error = anyhow::Error;
    fn try_from(v: u8) -> Result<Self> {
        match v {
            0 => Ok(BuildType::Cmd),
            1 => Ok(BuildType::Std),
            other => bail!("invalid build type: {}", other),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModBuild {
    pub id: String,
    pub name: String,
    pub git: String,
    pub build: BuildType,
    pub cmd: Option<String>,
    pub out: String,
    pub exclude: Vec<ExcludePair>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ExcludePair {
    #[serde(rename = "type")]
    pub type_name: ExcludeType,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[repr(u8)]
pub enum ExcludeType {
    Ends = 0,
    Starts = 1,
    Contains = 2,
}

impl TryFrom<u8> for ExcludeType {
    type Error = anyhow::Error;
    fn try_from(v: u8) -> Result<Self> {
        match v {
            0 => Ok(ExcludeType::Ends),
            1 => Ok(ExcludeType::Starts),
            2 => Ok(ExcludeType::Contains),
            other => bail!("invalid build type: {}", other),
        }
    }
}
