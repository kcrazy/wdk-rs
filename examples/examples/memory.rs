#![no_std]
#![no_main]

extern crate alloc;

use alloc::rc::Rc;
use core::alloc::{GlobalAlloc, Layout};

use wdk::allocator::KernelAllocator;
use wdk::println;
use wdk_sys::base::{DRIVER_OBJECT, NTSTATUS, STATUS_SUCCESS, UNICODE_STRING, _POOL_TYPE};

static ALLOCATOR: KernelAllocator =
    KernelAllocator::new(u32::from_ne_bytes(*b"kmem"), _POOL_TYPE::PagedPool);

#[no_mangle]
extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _: &UNICODE_STRING) -> NTSTATUS {
    driver.DriverUnload = Some(driver_exit);

    let x = unsafe {
        let ptr = ALLOCATOR.alloc(Layout::new::<u32>()) as *mut u32;
        *ptr = 5;
        Rc::from_raw(ptr)
    };

    let y = x.clone();

    let z = Rc::new(5000 as u128);

    println!("{} {} {}", x, y, z);

    STATUS_SUCCESS
}

extern "stdcall" fn driver_exit(_driver: *mut DRIVER_OBJECT) {}
