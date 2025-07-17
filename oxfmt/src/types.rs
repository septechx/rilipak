use anyhow::{Result, bail};
use downcast_rs::{Downcast, impl_downcast};
use std::{
    alloc::{Layout, alloc},
    any::Any,
    collections::HashMap,
    mem::MaybeUninit,
    ptr::copy_nonoverlapping,
    slice,
};

pub trait Serializable {
    fn serialize(&self) -> Result<Box<[u8]>>;
}

impl Serializable for String {
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

impl Serializable for u8 {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new(self.to_le_bytes()))
    }
}

impl Serializable for u16 {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new(self.to_le_bytes()))
    }
}

impl Serializable for u32 {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new(self.to_le_bytes()))
    }
}

impl Serializable for u64 {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new(self.to_le_bytes()))
    }
}

impl Serializable for u128 {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new(self.to_le_bytes()))
    }
}

impl Serializable for usize {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::new(self.to_le_bytes()))
    }
}

impl Serializable for Box<[u8]> {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(Box::clone(self))
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self) -> Result<Box<[u8]>> {
        let serialized: Vec<Box<[u8]>> = self
            .iter()
            .map(|item| item.serialize())
            .collect::<Result<_>>()?;
        let count = self.len();
        let payload_len: usize = serialized.iter().map(|b| b.len()).sum();
        let total_len = size_of::<usize>() + payload_len;

        let mut boxed_uninit: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(total_len);
        let ptr = boxed_uninit.as_mut_ptr() as *mut u8;

        unsafe {
            copy_nonoverlapping(count.to_le_bytes().as_ptr(), ptr, size_of::<usize>());
            let mut offset = size_of::<usize>();
            for b in serialized {
                copy_nonoverlapping(b.as_ptr(), ptr.add(offset), b.len());
                offset += b.len();
            }
            Ok(boxed_uninit.assume_init())
        }
    }
}

impl<T: Serializable> Serializable for Option<T> {
    fn serialize(&self) -> Result<Box<[u8]>> {
        Ok(match self {
            Some(val) => val.serialize()?,
            None => Box::from([]),
        })
    }
}

// Cursed code for runtime reflection
pub trait Deserializable: Downcast {
    fn get_structure() -> Structure
    where
        Self: Sized;
    fn construct(fields: Vec<Box<dyn Any>>) -> Result<Self>
    where
        Self: Sized;
}
impl_downcast!(Deserializable);

pub struct Structure {
    pub fields: HashMap<usize, Field>,
}

pub enum Field {
    String,
    U8,
    U16,
    U32,
    U64,
    U128,
    Struct,
    Vector,
}
