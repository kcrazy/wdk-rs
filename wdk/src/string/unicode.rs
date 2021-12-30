use alloc::vec::Vec;
use core::mem::size_of;
use fallible_collections::{FallibleVec};

use wdk_sys::base::UNICODE_STRING;

use crate::error::Error;

#[macro_export]
macro_rules! unicode_string {
    ($s:expr) => {{
        use wdk_sys::base::USHORT;

        const BUF:&[u16] = $crate::string::encode!($s);
        UNICODE_STRING {
            Length: (BUF.len() * 2) as USHORT,
            MaximumLength: (BUF.len() * 2) as USHORT,
            Buffer: BUF.as_ptr() as _,
        }
    }};
}

struct UnicodeString {
    buffer: Vec<u16>,
}

impl UnicodeString {
    fn from_utf16(utf16: &[u16]) -> Result<Self, Error> {
        let mut vec = Vec::try_with_capacity(utf16.len())?;
        vec.copy_from_slice(utf16);

        Ok(UnicodeString {
            buffer: vec,
        })
    }

    fn to_unicode_string(&self) -> UNICODE_STRING {
        let length = (self.buffer.len() * size_of::<u16>()) as u16;

        UNICODE_STRING {
            Length: length,
            MaximumLength: length,
            Buffer: self.buffer.as_ptr() as _,
        }
    }
}