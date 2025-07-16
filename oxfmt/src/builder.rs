use crate::types::Serializable;
use anyhow::Result;

pub struct BinaryBuilder {
    buf: Vec<u8>,
}

impl BinaryBuilder {
    pub fn new(header: &[u8], version: u16) -> Self {
        let mut buf = Vec::new();
        buf.extend(header);
        buf.extend(version.to_le_bytes());
        let arch: u8 = match size_of::<usize>() {
            4 => 0,
            8 => 1,
            _ => panic!("invalid architecture: {}", size_of::<usize>()),
        };
        buf.push(arch);
        Self { buf }
    }

    pub fn new_no_meta() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn add<T: Serializable>(mut self, serializeable: &T) -> Result<Self> {
        self.buf.extend(serializeable.serialize()?);
        Ok(self)
    }

    pub fn build(self) -> Box<[u8]> {
        self.buf.into_boxed_slice()
    }
}
