use std::any::Any;

use anyhow::Result;
use oxfmt::{Deserializable, Field, Serializable, Structure, construct, structure};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Serializable)]
#[repr(u8)]
pub enum BuildType {
    Cmd = 0,
    Std = 1,
}

impl From<u8> for BuildType {
    fn from(v: u8) -> Self {
        match v {
            0 => BuildType::Cmd,
            1 => BuildType::Std,
            other => panic!("invalid build type: {other}"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Serializable)]
#[oxfmt(header = "mcmodbuild", version = 1)]
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Serializable)]
pub struct ExcludePair {
    #[serde(rename = "type")]
    pub type_name: ExcludeType,
    pub value: String,
}

impl Deserializable for ExcludePair {
    fn get_structure() -> Structure {
        structure!(Field::U8, Field::String)
    }

    fn construct(mut fields: Vec<Box<dyn Any>>) -> Result<Self> {
        construct!(fields,
            type_name: ExcludeType as u8,
            value: String as String,
        )
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Serializable)]
#[repr(u8)]
pub enum ExcludeType {
    Ends = 0,
    Starts = 1,
    Contains = 2,
}

impl From<u8> for ExcludeType {
    fn from(v: u8) -> Self {
        match v {
            0 => ExcludeType::Ends,
            1 => ExcludeType::Starts,
            2 => ExcludeType::Contains,
            other => panic!("invalid build type: {other}"),
        }
    }
}
