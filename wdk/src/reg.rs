use crate::error::Error;
use alloc::vec;
use alloc::vec::Vec;
use core::mem;
use core::ptr::null_mut;
use core::slice;

use crate::string::UnicodeString;
use wdk_sys::base::{
    HANDLE, KEY_VALUE_PARTIAL_INFORMATION, OBJECT_ATTRIBUTES, REG_DWORD, REG_MULTI_SZ,
    STATUS_BUFFER_OVERFLOW, STATUS_INSUFFICIENT_RESOURCES, UNICODE_STRING,
    _KEY_VALUE_INFORMATION_CLASS, _POOL_TYPE,
};
use wdk_sys::ntoskrnl::{
    ExAllocatePoolWithTag, ExFreePoolWithTag, ZwClose, ZwOpenKey, ZwQueryValueKey,
};

const REG_TAG: u32 = u32::from_ne_bytes(*b"rust");

pub struct RegKey {
    hkey: HANDLE,
}

impl Drop for RegKey {
    fn drop(&mut self) {
        unsafe {
            ZwClose(self.hkey);
        }
    }
}

impl RegKey {
    pub fn open(path: &UNICODE_STRING) -> Result<Self, Error> {
        let mut handle: HANDLE = null_mut();
        unsafe {
            let mut us = path.clone();
            let mut oa: OBJECT_ATTRIBUTES = OBJECT_ATTRIBUTES {
                Length: mem::size_of::<OBJECT_ATTRIBUTES>() as _,
                RootDirectory: null_mut(),
                ObjectName: &mut us,
                Attributes: 0,
                SecurityDescriptor: null_mut(),
                SecurityQualityOfService: null_mut(),
            };
            let ns = ZwOpenKey(&mut handle, 0, &mut oa);
            if ns < 0 {
                return Err(Error::from_ntstatus(ns));
            }
        }
        Ok(RegKey { hkey: handle })
    }

    pub fn get_value(&self, path: &UNICODE_STRING) -> Result<RegValue, Error> {
        unsafe {
            let mut value_info: KEY_VALUE_PARTIAL_INFORMATION = mem::zeroed();
            let mut retlen: u32 = 0;
            let mut us = path.clone();
            let ns = ZwQueryValueKey(
                self.hkey,
                &mut us,
                _KEY_VALUE_INFORMATION_CLASS::KeyValuePartialInformation,
                &mut value_info as *mut KEY_VALUE_PARTIAL_INFORMATION as _,
                mem::size_of::<KEY_VALUE_PARTIAL_INFORMATION>() as _,
                &mut retlen,
            );
            if ns != STATUS_BUFFER_OVERFLOW {
                return Err(Error::from_ntstatus(ns));
            }

            let kvpi: *mut KEY_VALUE_PARTIAL_INFORMATION =
                ExAllocatePoolWithTag(_POOL_TYPE::PagedPool, retlen, REG_TAG) as _;
            if kvpi == null_mut() {
                return Err(Error::from_ntstatus(STATUS_INSUFFICIENT_RESOURCES));
            }

            let ns = ZwQueryValueKey(
                self.hkey,
                &mut us,
                _KEY_VALUE_INFORMATION_CLASS::KeyValuePartialInformation,
                kvpi as _,
                retlen,
                &mut retlen,
            );
            if ns < 0 {
                ExFreePoolWithTag(kvpi as _, REG_TAG);
                return Err(Error::from_ntstatus(ns));
            }

            let value = match (*kvpi).Type {
                REG_DWORD => RegValue::RegDword(*((*kvpi).Data.as_ptr() as *const u32)),
                REG_MULTI_SZ => {
                    let mut words = slice::from_raw_parts(
                        (*kvpi).Data.as_ptr() as *const u16,
                        ((*kvpi).DataLength / 2) as _,
                    );
                    while let Some(0) = words.last() {
                        words = &words[0..words.len() - 1];
                    }

                    let mut v = vec![];
                    for w in words.split(|ch| *ch == 0u16) {
                        v.push(UnicodeString::from_utf16(w)?);
                    }

                    RegValue::RegMultiSz(v)
                }
                _ => todo!(),
            };
            ExFreePoolWithTag(kvpi as _, REG_TAG);
            Ok(value)
        }
    }
}

pub enum RegValue {
    RegDword(u32),
    RegSz(UnicodeString),
    RegMultiSz(Vec<UnicodeString>),
}
