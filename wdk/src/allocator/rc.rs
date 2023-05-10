use core::alloc::Layout;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

pub use wdk_sys::base::POOL_TYPE;
use wdk_sys::base::STATUS_INSUFFICIENT_RESOURCES;
use wdk_sys::ntoskrnl::{ExAllocatePoolWithTag, ExFreePoolWithTag};

use crate::error::Error;

pub struct Rc<T> {
    ptr: *mut RcNode<T>,
    _marker: PhantomData<RcNode<T>>,
}

struct RcNode<T> {
    value: T,
    refcount: AtomicUsize,
    tag: u32,
}

unsafe impl<T> Send for Rc<T> {}
unsafe impl<T> Sync for Rc<T> {}

impl<T> Rc<T> {
    pub fn new(value: T, pool_type: POOL_TYPE, tag: u32) -> Result<Self, Error> {
        let rc_node = RcNode {
            value,
            refcount: AtomicUsize::new(1),
            tag,
        };

        let layout = Layout::new::<RcNode<T>>();

        unsafe {
            let ptr = ExAllocatePoolWithTag(pool_type, layout.size() as _, tag);
            if ptr.is_null() {
                Err(Error::from_ntstatus(STATUS_INSUFFICIENT_RESOURCES))
            } else {
                *(ptr as *mut RcNode<T>) = rc_node;
                Ok(Rc {
                    ptr: ptr as *mut RcNode<T>,
                    _marker: PhantomData,
                })
            }
        }
    }

    pub fn clone(&self) -> Self {
        let _old_refcount = unsafe { &(*self.ptr) }
            .refcount
            .fetch_add(1, Ordering::SeqCst);
        Rc {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let rc_node = unsafe { &mut *self.ptr };
        let new_refcount = rc_node.refcount.fetch_sub(1, Ordering::SeqCst) - 1;
        if new_refcount == 0 {
            unsafe { ExFreePoolWithTag(self.ptr as _, rc_node.tag) };
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let rc_node = unsafe { &mut *self.ptr };
        &rc_node.value
    }
}

impl<T> DerefMut for Rc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let rc_node = unsafe { &mut *self.ptr };
        &mut rc_node.value
    }
}
