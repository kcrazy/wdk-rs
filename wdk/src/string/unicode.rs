use core::char::{decode_utf16, REPLACEMENT_CHARACTER};
use core::fmt;
use core::fmt::Write;
use core::mem::size_of;
use core::slice;

use fallible_collections::{FallibleVec, TryCollect};

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
}