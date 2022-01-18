use wdk_sys::base::UNICODE_STRING;
use wdk_sys::ntoskrnl::{IoCreateSymbolicLink, IoDeleteSymbolicLink};

use crate::error::{Error, IntoResult};
use crate::string::UnicodeString;

pub enum SymbolicLink {
    Name { name: UnicodeString },
    ConstName { name: UNICODE_STRING },
}

impl SymbolicLink {
    pub fn new(name: UnicodeString, target: &UnicodeString) -> Result<Self, Error> {
        unsafe {
            IoCreateSymbolicLink(
                &mut name.to_unicode_string(),
                &mut target.to_unicode_string(),
            )
        }
        .into_result()?;

        Ok(SymbolicLink::Name { name })
    }

    pub fn from_const(mut name: UNICODE_STRING, target: &UnicodeString) -> Result<Self, Error> {
        unsafe { IoCreateSymbolicLink(&mut name, &mut target.to_unicode_string()) }
            .into_result()?;

        Ok(SymbolicLink::ConstName { name })
    }
}

impl Drop for SymbolicLink {
    fn drop(&mut self) {
        let mut us = match self {
            SymbolicLink::Name { name } => name.to_unicode_string(),
            SymbolicLink::ConstName { name } => *name,
        };

        unsafe {
            IoDeleteSymbolicLink(&mut us);
        }
    }
}
