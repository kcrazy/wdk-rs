use core::alloc::Layout;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

pub use wdk_sys::base::_POOL_TYPE as POOL_TYPE;
use wdk_sys::ntoskrnl::{ExAllocatePoolWithTag, ExFreePoolWithTag};

pub struct Box<T> {
    tag: u32,
    data: NonNull<T>,
}

impl<T> Box<T> {
    pub fn new(data: T, tag: u32) -> Option<Self> {
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = ExAllocatePoolWithTag(POOL_TYPE::NonPagedPool, layout.size() as u64, tag);
            if ptr.is_null() {
                None
            } else {
                let mut ptr = NonNull::<T>::new(ptr as *mut T).unwrap();
                *(ptr.as_mut()) = data;
                Some(Box { tag, data: ptr })
            }
        }
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe { ExFreePoolWithTag(self.data.as_ptr() as _, self.tag) }
    }
}

impl<T> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}

impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.data.as_mut() }
    }
}
