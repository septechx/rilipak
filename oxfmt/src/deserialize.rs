use std::any::Any;

use anyhow::{bail, Result};

use crate::types::{Deserializable, Field};

pub struct Deserialize<'a> {
    buf: &'a [u8],
    arch: Option<u8>,
}

impl<'a> Deserialize<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, arch: None }
    }

    pub fn read_string(&mut self) -> Result<String> {
        match self.buf.iter().position(|&byte| byte == 0) {
            Some(pos) => {
                let string = str::from_utf8(&self.buf[..pos])?.to_string();
                self.advance(pos + 1);
                Ok(string)
            }
            None => {
                bail!("unterminated c-string")
            }
        }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let bytes = self.read_bytes(1)?;
        Ok(u8::from_le_bytes(bytes.try_into()?))
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let bytes = self.read_bytes(2)?;
        Ok(u16::from_le_bytes(bytes.try_into()?))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let bytes = self.read_bytes(4)?;
        Ok(u32::from_le_bytes(bytes.try_into()?))
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        let bytes = self.read_bytes(8)?;
        Ok(u64::from_le_bytes(bytes.try_into()?))
    }

    pub fn read_u128(&mut self) -> Result<u128> {
        let bytes = self.read_bytes(16)?;
        Ok(u128::from_le_bytes(bytes.try_into()?))
    }

    pub fn read_struct<T: Deserializable>(&mut self) -> Result<T> {
        let structure = T::get_structure();

        let mut fields = Vec::new();

        for i in 0..structure.fields.len() {
            let field: Box<dyn Any> = match structure.fields.get(&i) {
                Some(field) => match field {
                    Field::String => Box::new(self.read_string()?),
                    Field::U8 => Box::new(self.read_u8()?),
                    Field::U16 => Box::new(self.read_u16()?),
                    Field::U32 => Box::new(self.read_u32()?),
                    Field::U64 => Box::new(self.read_u64()?),
                    Field::U128 => Box::new(self.read_u128()?),
                    Field::Struct => Box::new(self.read_struct::<T>()?),
                    Field::Vector => Box::new(self.read_vec::<T>()?),
                },
                None => bail!("field index {} not found", i),
            };
            fields.push(field);
        }

        T::construct(fields)
    }

    pub fn read_vec<T: Deserializable>(&mut self) -> Result<Vec<T>> {
        let size = self.read_usize()?;
        let mut vec: Vec<T> = Vec::with_capacity(size);

        for _ in 0..size {
            vec.push(self.read_struct()?);
        }

        Ok(vec)
    }

    pub fn read_usize(&mut self) -> Result<usize> {
        let size = match self.arch {
            Some(arch) => match arch {
                0 => 4, // 32-bit
                1 => 8, // 64-bit
                _ => bail!("invalid architecture {}", arch),
            },
            None => bail!("no architecture specified"),
        };

        let bytes = self.read_bytes(size)?;

        let value = match size {
            4 => {
                let arr: [u8; 4] = bytes.try_into()?;
                u32::from_le_bytes(arr) as u64
            }
            8 => {
                let arr: [u8; 8] = bytes.try_into()?;
                u64::from_le_bytes(arr)
            }
            _ => bail!("unsupported size: {}", size),
        };

        let usize_val =
            usize::try_from(value).map_err(|_| anyhow::anyhow!("value does not fit in usize"))?;

        if usize::BITS < 64 && value > u32::MAX as u64 {
            bail!("value does not fit in 32-bit usize");
        }

        Ok(usize_val)
    }
    pub fn read_bytes(&mut self, bytes: usize) -> Result<&[u8]> {
        if self.buf.len() < bytes {
            bail!("not enough bytes")
        }
        let result = &self.buf[..bytes];
        self.advance(bytes);
        Ok(result)
    }

    fn advance(&mut self, len: usize) {
        self.buf = &self.buf[len..];
    }

    pub fn assert_header(&mut self, header: &[u8]) -> Result<()> {
        if !self.buf.starts_with(header) {
            bail!("buffer does not start with header")
        }
        self.advance(header.len());
        Ok(())
    }

    pub fn assert_version(&mut self, version: u16) -> Result<()> {
        if self.read_u16()? != version {
            bail!("version does not match")
        }
        Ok(())
    }

    pub fn read_arch(&mut self) -> Result<()> {
        self.arch = Some(self.read_u8()?);
        Ok(())
    }

    pub fn init(mut self, header: &[u8], version: u16) -> Result<Self> {
        self.assert_header(header)?;
        self.assert_version(version)?;
        self.read_arch()?;
        Ok(self)
    }
}
