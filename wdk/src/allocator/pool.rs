use cty::c_void;

use wdk_sys::ntoskrnl::{ExAllocatePoolWithTag, ExFreePoolWithTag};
pub use wdk_sys::base::POOL_TYPE;
use wdk_sys::base::STATUS_INSUFFICIENT_RESOURCES;

use crate::error::Error;

pub struct Pool {
    tag: u32,
    data: *const c_void,
}

impl Pool {
    pub fn new(len: u32, type_: POOL_TYPE, tag: u32) -> Result<Self, Error> {

        unsafe {
            let ptr = ExAllocatePoolWithTag(type_, len as _, tag);
            if ptr.is_null() {
                Err(Error::from_ntstatus(STATUS_INSUFFICIENT_RESOURCES))
            } else {
                Ok(Pool { tag, data: ptr })
            }
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        unsafe { ExFreePoolWithTag(self.data as _, self.tag) }
    }
}