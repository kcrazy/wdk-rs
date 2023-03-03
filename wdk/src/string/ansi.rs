use core::mem::size_of;

use wdk_sys::base::{ANSI_STRING, UNICODE_STRING};
use wdk_sys::ntoskrnl::RtlUnicodeStringToAnsiString;

use crate::allocator::vec::Vec;
use crate::allocator::POOL_TYPE;
use crate::error::Error;
use crate::string::UnicodeString;

pub struct AnsiString {
    buffer: Vec<u8>,
}

impl AnsiString {
    pub fn from_utf16(utf16: &[u16], pool_type: POOL_TYPE, tag: u32) -> Result<Self, Error> {
        let length = utf16.len() * size_of::<u16>();
        let vec = Vec::with_capacity(length, pool_type, tag)?;

        let mut ansi = ANSI_STRING {
            Length: 0,
            MaximumLength: length as u16,
            Buffer: vec.as_ptr() as _,
        };

        let unicode = UNICODE_STRING {
            Length: length as u16,
            MaximumLength: length as u16,
            Buffer: utf16.as_ptr() as _,
        };

        unsafe {
            RtlUnicodeStringToAnsiString(&mut ansi as _, &unicode, 0);
        };

        Ok(AnsiString { buffer: vec })
    }

    pub fn from_str(s: &str, pool_type: POOL_TYPE, tag: u32) -> Result<Self, Error> {
        let us = UnicodeString::from_str(s, pool_type, tag)?;
        us.to_ansi(pool_type, tag)
    }

    pub fn to_ansi_string(&self) -> ANSI_STRING {
        let length = (self.buffer.len() * size_of::<u8>()) as u16;

        ANSI_STRING {
            Length: length,
            MaximumLength: length,
            Buffer: self.buffer.as_ptr() as _,
        }
    }
}
