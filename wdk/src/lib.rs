#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod allocator;
pub mod dbg;
pub mod error;
pub mod version;

use core::panic::PanicInfo;

#[cfg(feature = "alloc")]
#[global_allocator]
static ALLOCATOR: allocator::KernelAllocator = allocator::KernelAllocator::new(
    u32::from_ne_bytes(*b"rust")
);

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "system" fn __CxxFrameHandler3() -> i32 {
    0
}

#[cfg(target_arch = "x86")]
#[link_name = "___CxxFrameHandler3@0"]
pub extern "system" fn __cxx_frame_handler3_0() -> i32 {
    0
}