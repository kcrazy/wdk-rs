mod deref;
mod drop;
mod from_iterator;
mod helpers;

use core::marker::PhantomData;
use core::ptr::NonNull;

use wdk_sys::base::{STATUS_BUFFER_OVERFLOW, STATUS_INSUFFICIENT_RESOURCES};
use wdk_sys::ntoskrnl::{ExAllocatePoolWithTag, ExFreePoolWithTag};

use crate::allocator::POOL_TYPE;
use crate::error::Error;

use helpers::*;

pub use from_iterator::TryCollect;

#[repr(transparent)]
pub struct Vec<T> {
    buf: NonNull<u8>,
    phantom: PhantomData<T>,
}

#[derive(Clone, Copy)]
struct Header {
    len: usize,
    cap: usize,
    pool_type: POOL_TYPE,
    tag: u32,
}

impl<T> Vec<T> {
    const N: usize = next_aligned(core::mem::size_of::<Header>(), max_align::<T>());

    pub fn new(pool_type: POOL_TYPE, tag: u32) -> Result<Vec<T>, Error> {
        Vec::with_capacity(0, pool_type, tag)
    }

    pub fn with_capacity(capacity: usize, pool_type: POOL_TYPE, tag: u32) -> Result<Vec<T>, Error> {
        assert!(
            core::mem::size_of::<T>() > 0,
            "ZSTs currently not supported"
        );

        let capacity = if capacity == 0 {
            next_capacity::<T>(0)
        } else {
            capacity
        };

        let layout = make_layout::<T>(capacity);
        let buf = unsafe { ExAllocatePoolWithTag(pool_type, layout.size() as _, tag) };
        if buf.is_null() {
            return Err(Error::from_ntstatus(STATUS_INSUFFICIENT_RESOURCES));
        }

        let header = Header {
            len: 0,
            cap: capacity,
            pool_type,
            tag,
        };

        unsafe { core::ptr::write(buf.cast::<Header>(), header) };

        Ok(Vec {
            buf: unsafe { core::ptr::NonNull::new_unchecked(buf as _) },
            phantom: core::marker::PhantomData,
        })
    }

    pub fn reserve(&mut self, additional: usize) -> Result<(), Error> {
        let capacity = self.capacity();
        let total_required = self.len().saturating_add(additional);

        if total_required <= capacity {
            return Ok(());
        }

        let mut new_capacity = next_capacity::<T>(capacity);
        while new_capacity < total_required {
            new_capacity = next_capacity::<T>(new_capacity);
        }

        let max_elems = max_elems::<T>();

        if !self.is_empty() && total_required > max_elems {
            return Err(Error::from_ntstatus(STATUS_BUFFER_OVERFLOW));
        }

        if additional > max_elems {
            new_capacity = max_elems;
        }

        self.grow(new_capacity)
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        self.header_mut().len = len;
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.len();

        if len == 0 {
            return None;
        }

        let v = unsafe { core::ptr::read(self.as_ptr().add(len - 1)) };
        unsafe {
            self.set_len(len - 1);
        }
        Some(v)
    }

    pub fn push(&mut self, value: T) -> Result<&mut T, Error> {
        let (len, capacity) = (self.len(), self.capacity());
        if len == capacity {
            self.grow(next_capacity::<T>(capacity))?
        }

        let len = self.len();
        let data = self.data();

        let dst = unsafe { data.add(len) };

        unsafe {
            core::ptr::write(dst, value);
        };

        let mut header = self.header_mut();
        header.len += 1;

        Ok(unsafe { &mut *dst })
    }

    pub fn as_ptr(&self) -> *const T {
        self.data()
    }

    pub fn as_slice(&self) -> &[T] {
        self
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }

    pub fn capacity(&self) -> usize {
        self.header().cap
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.header().len
    }

    fn header(&self) -> &Header {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            &*(self.buf.as_ptr() as *const Header)
        }
    }

    fn header_mut(&mut self) -> &mut Header {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            &mut *self.buf.as_ptr().cast::<Header>()
        }
    }

    fn data(&self) -> *mut T {
        unsafe { self.buf.as_ptr().add(Self::N).cast::<T>() }
    }

    fn grow(&mut self, capacity: usize) -> Result<(), Error> {
        debug_assert!(capacity >= self.len());

        let old_capacity = self.capacity();
        let new_capacity = capacity;

        if new_capacity == old_capacity {
            return Ok(());
        }

        let new_layout = make_layout::<T>(new_capacity);

        let len = self.len();

        let new_buf = {
            unsafe {
                ExAllocatePoolWithTag(
                    self.header().pool_type,
                    new_layout.size() as _,
                    self.header().tag,
                )
            }
            //unsafe { alloc::alloc::realloc(self.buf.as_ptr(), old_layout, new_layout.size()) }
        };

        if new_buf.is_null() {
            return Err(Error::from_ntstatus(STATUS_INSUFFICIENT_RESOURCES));
        }

        unsafe {
            core::ptr::write(new_buf as _, self.buf.as_ptr());
            ExFreePoolWithTag(self.buf.as_ptr() as _, self.header().tag)
        }

        let header = Header {
            len,
            cap: new_capacity,
            pool_type: self.header().pool_type,
            tag: self.header().tag,
        };

        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            core::ptr::write(new_buf.cast::<Header>(), header);
        }

        self.buf = unsafe { core::ptr::NonNull::<u8>::new_unchecked(new_buf as _) };

        Ok(())
    }
}

impl<T: Clone> Vec<T> {
    pub fn extend_from_slice(&mut self, elems: &[T]) -> Result<(), Error> {
        self.reserve(elems.len())?;
        for x in elems {
            self.push((*x).clone())?;
        }
        Ok(())
    }
}

/// `mini_vec!` is a macro similar in spirit to the stdlib's `vec!`.
///
/// It supports the creation of `MiniVec` with:
/// * `mini_vec!()`
/// * `mini_vec![val1, val2, val3, ...]`
/// * `mini_vec![val; num_elems]`
///
#[macro_export]
macro_rules! vec {
    () => (
        $crate::Vec::new()
    );
    ($elem:expr; $n:expr) => {
        {
            let len = $n;
            let mut tmp = $crate::Vec::with_capacity(len);

            for idx in 0..len {
                unsafe { tmp.unsafe_write(idx, $elem.clone()) };
            }


            if len > 0 {
                unsafe { tmp.set_len(len) };
            }

            tmp
        }
     };
    ($($x:expr),+ $(,)?) => {
        {
            let mut tmp = $crate::Vec::new();
            $(
                tmp.push($x);
            )*
            tmp
        }
    };
}
