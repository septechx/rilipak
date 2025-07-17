use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PackConfig {
    pub id: String,
    pub name: String,
    pub author: String,
    pub mods: Vec<Mod>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Mod {
    pub source: ModSource,
    pub id: String,
    pub env: ModEnv,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ModSource {
    Curseforge,
    Modrinth,
    Github,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ModEnv {
    Server,
    Client,
    Common,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pack {
    pub meta: PackMeta,
    pub include: Box<[u8]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PackMeta {
    pub config: PackConfig,
    pub modbuilds: Vec<Box<[u8]>>,
    pub kube: KubeBundle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KubeBundle {
    pub server: String,
    pub client: String,
}
