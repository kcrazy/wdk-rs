#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod allocator;
pub mod dbg;
pub mod device;
pub mod driver;
pub mod error;
pub mod ioctl;
pub mod request;
pub mod string;
pub mod symbolic_link;
pub mod sync;
pub mod user_ptr;
pub mod version;
pub mod reg;

use core::panic::PanicInfo;

use wdk_sys::base::STATUS_ACCESS_VIOLATION;
use wdk_sys::ntoskrnl::KeBugCheck;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        KeBugCheck(STATUS_ACCESS_VIOLATION as u32);
    }
    loop {}
}

#[used]
#[no_mangle]
static _fltused: i32 = 0;

#[used]
#[no_mangle]
static __security_cookie: i32 = 88888888;

#[cfg(target_arch = "x86_64")]
#[no_mangle]
extern "system" fn __CxxFrameHandler3() -> i32 {
    0
}

#[cfg(target_arch = "x86")]
#[no_mangle]
fn __CxxFrameHandler3() -> i32 {
    0
}
