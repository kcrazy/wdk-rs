use core::mem::size_of;

use wdk_sys::base::{ANSI_STRING, STATUS_INSUFFICIENT_RESOURCES, UNICODE_STRING};
use wdk_sys::ntoskrnl::{ExAllocatePoolWithTag, ExFreePoolWithTag, RtlUnicodeStringToAnsiString};

use crate::allocator::POOL_TYPE;
use crate::error::Error;
use crate::string::UnicodeString;

pub struct AnsiString {
    buffer: *const u8,
    len: u16,
    tag: u32,
}

impl AnsiString {
    pub fn from_utf16(utf16: &[u16], type_: POOL_TYPE, tag: u32) -> Result<Self, Error> {
        let length = utf16.len() * size_of::<u16>();

        let ptr = unsafe {
            let ptr = ExAllocatePoolWithTag(type_, length as _, tag);
            if ptr.is_null() {
                return Err(Error::from_ntstatus(STATUS_INSUFFICIENT_RESOURCES));
            }
            ptr
        };

        let mut ansi = ANSI_STRING {
            Length: 0,
            MaximumLength: length as u16,
            Buffer: ptr as _,
        };

        let unicode = UNICODE_STRING {
            Length: length as u16,
            MaximumLength: length as u16,
            Buffer: utf16.as_ptr() as _,
        };

        unsafe {
            RtlUnicodeStringToAnsiString(&mut ansi as _, &unicode, 0);
        };

        Ok(AnsiString {
            buffer: ptr as _,
            len: length as _,
            tag,
        })
    }

    // pub fn from_str(s: &str) -> Result<Self, Error> {
    //     let us = UnicodeString::from_str(s)?;
    //     us.to_ansi()
    // }

    pub fn to_ansi_string(&self) -> ANSI_STRING {
        ANSI_STRING {
            Length: self.len,
            MaximumLength: self.len,
            Buffer: self.buffer as _,
        }
    }
}

impl Drop for AnsiString {
    fn drop(&mut self) {
        unsafe { ExFreePoolWithTag(self.buffer as _, self.tag) }
    }
}
