use anyhow::{bail, Result};
use std::{
    alloc::{alloc, Layout},
    ptr::copy_nonoverlapping,
    slice,
};

pub trait Serializeable {
    fn serialize(&self) -> Result<Box<[u8]>>;
}

impl Serializeable for String {
    fn serialize(&self) -> Result<Box<[u8]>> {
        let len = self.len() + 1;
        let layout = Layout::array::<u8>(len)?;

        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            bail!("allocation failed");
        }

        unsafe {
            copy_nonoverlapping(self.as_ptr(), ptr, self.len());
            *ptr.add(self.len()) = 0;

            let slice = slice::from_raw_parts_mut(ptr, len);
            Ok(Box::from_raw(slice))
        }
    }
}

impl Serializeable for u8 {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new(self.to_le_bytes()))
    }
}

impl Serializeable for u16 {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new(self.to_le_bytes()))
    }
}

impl Serializeable for u32 {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new(self.to_le_bytes()))
    }
}

impl Serializeable for u64 {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new(self.to_le_bytes()))
    }
}

impl Serializeable for u128 {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new(self.to_le_bytes()))
    }
}

impl<T: Serializeable> Serializeable for Vec<T> {
    fn serialize(&self) -> Result<Box<[u8]>> {
        if self.len() > u8::MAX as usize {
            bail!("Vec length exceeds u8 maximum");
        }

        let mut result = Vec::with_capacity(1);
        result.push(self.len() as u8);

        for item in self {
            let bytes = item.serialize()?;
            result.extend_from_slice(&bytes);
        }

        Ok(result.into_boxed_slice())
    }
}

impl<T: Serializeable> Serializeable for Option<T> {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(match self {
            Some(val) => val.serialize()?,
            None => Box::from([]),
        })
    }
}
