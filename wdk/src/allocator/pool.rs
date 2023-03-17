use core::alloc::Layout;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

pub use wdk_sys::base::POOL_TYPE;
use wdk_sys::base::STATUS_INSUFFICIENT_RESOURCES;
use wdk_sys::ntoskrnl::{ExAllocatePoolWithTag, ExFreePoolWithTag};

use crate::error::Error;

pub struct Pool<T: ?Sized> {
    tag: u32,
    data: NonNull<T>,
}

impl<T> Pool<T> {
    pub fn new(data: T, pool_type: POOL_TYPE, tag: u32) -> Result<Self, Error> {
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = ExAllocatePoolWithTag(pool_type, layout.size() as _, tag);
            if ptr.is_null() {
                Err(Error::from_ntstatus(STATUS_INSUFFICIENT_RESOURCES))
            } else {
                let mut ptr = NonNull::<T>::new(ptr as *mut T).unwrap();
                *(ptr.as_mut()) = data;
                Ok(Pool { tag, data: ptr })
            }
        }
    }
}

impl<T: ?Sized> Drop for Pool<T> {
    fn drop(&mut self) {
        unsafe { ExFreePoolWithTag(self.data.as_ptr() as _, self.tag) }
    }
}

impl<T> Deref for Pool<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}

impl<T> DerefMut for Pool<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.data.as_mut() }
    }
}
