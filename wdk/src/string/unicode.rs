use alloc::string::String;
use alloc::vec::Vec;
use core::char::{decode_utf16, REPLACEMENT_CHARACTER};
use core::fmt;
use core::mem::size_of;
use fallible_collections::FallibleVec;

use wdk_sys::base::UNICODE_STRING;

use crate::error::Error;
use crate::string::ansi::AnsiString;

#[macro_export]
macro_rules! unicode_string {
    ($s:expr) => {{
        use wdk_sys::base::USHORT;

        const BUF: &[u16] = $crate::string::encode!($s);
        UNICODE_STRING {
            Length: (BUF.len() * 2) as USHORT,
            MaximumLength: (BUF.len() * 2) as USHORT,
            Buffer: BUF.as_ptr() as _,
        }
    }};
}

pub struct UnicodeString {
    buffer: Vec<u16>,
}

impl UnicodeString {
    pub fn from_utf16(utf16: &[u16]) -> Result<Self, Error> {
        let mut vec = Vec::try_with_capacity(utf16.len())?;
        vec.extend_from_slice(utf16);

        Ok(UnicodeString { buffer: vec })
    }

    pub fn from_str(s: &str) -> Result<Self, Error> {
        //FIXME: OOM
        let utf16: Vec<u16> = s.encode_utf16().collect();

        Ok(UnicodeString { buffer: utf16 })
    }

    pub fn to_ansi(&self) -> Result<AnsiString, Error> {
        AnsiString::from_utf16(&self.buffer.as_slice())
    }

    pub fn to_unicode_string(&self) -> UNICODE_STRING {
        let length = (self.buffer.len() * size_of::<u16>()) as u16;

        UNICODE_STRING {
            Length: length,
            MaximumLength: length,
            Buffer: self.buffer.as_ptr() as _,
        }
    }
}

impl fmt::Display for UnicodeString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //FIXME: OOM
        let s: String = decode_utf16(self.buffer.iter().cloned())
            .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
            .collect();

        f.write_str(s.as_str())
    }
}
