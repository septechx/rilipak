use anyhow::{Result, bail};
use oxfmt::Serializeable;
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

impl Serializeable for BuildType {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new([*self as u8]))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModBuild {
    pub id: String,
    pub name: String,
    pub git: String,
    pub branch: String,
    pub build: BuildType,
    pub cmd: Option<String>,
    pub out: String,
    pub exclude: Vec<ExcludePair>,
}

impl Serializeable for ModBuild {
    fn serialize(&self) -> Result<Box<[u8]>> {
        let header_id: String = "mcmodbuild".to_string();
        let header_version: u16 = 1;

        let mut buf: Vec<u8> = vec![];
        buf.extend(Serializeable::serialize(&header_id)?);
        buf.extend(Serializeable::serialize(&header_version)?);
        buf.extend(Serializeable::serialize(&self.id)?);
        buf.extend(Serializeable::serialize(&self.name)?);
        buf.extend(Serializeable::serialize(&self.git)?);
        buf.extend(Serializeable::serialize(&self.branch)?);
        buf.extend(Serializeable::serialize(&self.build)?);
        buf.extend(Serializeable::serialize(&self.cmd)?);
        buf.extend(Serializeable::serialize(&self.out)?);
        buf.extend(Serializeable::serialize(&self.exclude)?);
        Ok(buf.into_boxed_slice())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ExcludePair {
    #[serde(rename = "type")]
    pub type_name: ExcludeType,
    pub value: String,
}

impl Serializeable for ExcludePair {
    fn serialize(&self) -> Result<Box<[u8]>> {
        let mut buf: Vec<u8> = vec![];
        buf.extend(Serializeable::serialize(&self.type_name)?);
        buf.extend(Serializeable::serialize(&self.value)?);
        Ok(buf.into_boxed_slice())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
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

impl Serializeable for ExcludeType {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new([*self as u8]))
    }
}
