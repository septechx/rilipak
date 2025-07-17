use std::{any::Any, collections::HashMap};

use anyhow::{Result, anyhow, bail};
use oxfmt::{BinaryBuilder, Deserializable, Field, Serializable, Structure};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Serializable)]
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
    pub branch: String,
    pub build: BuildType,
    pub cmd: Option<String>,
    pub out: String,
    pub exclude: Vec<ExcludePair>,
}

impl Serializable for ModBuild {
    fn serialize(&self) -> Result<Box<[u8]>> {
        let header = "mcmodbuild".as_bytes();
        let version: u16 = 1;

        let result = BinaryBuilder::new(header, version)
            .add(&self.id)?
            .add(&self.name)?
            .add(&self.git)?
            .add(&self.branch)?
            .add(&self.build)?
            .add(&self.cmd)?
            .add(&self.out)?
            .add(&self.exclude)?
            .build();

        Ok(result)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Serializable)]
pub struct ExcludePair {
    #[serde(rename = "type")]
    pub type_name: ExcludeType,
    pub value: String,
}

impl Deserializable for ExcludePair {
    fn get_structure() -> Structure {
        let mut fields = HashMap::new();
        fields.insert(0, Field::U8);
        fields.insert(1, Field::String);
        Structure { fields }
    }

    fn construct(mut fields: Vec<Box<dyn Any>>) -> Result<Self> {
        let type_value = *fields
            .remove(0)
            .downcast::<u8>()
            .map_err(|_| anyhow!("expected u8 for ExcludeType"))?;
        let type_name = ExcludeType::try_from(type_value)?;

        let value = *fields
            .remove(0)
            .downcast::<String>()
            .map_err(|_| anyhow!("expected string"))?;

        Ok(ExcludePair { type_name, value })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Serializable)]
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
