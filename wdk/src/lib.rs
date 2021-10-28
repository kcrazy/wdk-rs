#![no_std]

pub mod alloc;
pub mod dbg;
pub mod error;
pub mod version;

use core::panic::PanicInfo;

use wdk_sys::ntoskrnl::KeBugCheck;
use wdk_sys::base::STATUS_ACCESS_VIOLATION;

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

#[cfg(target_arch = "x86_64")]
#[no_mangle]
extern "system" fn __CxxFrameHandler3() -> i32 {
    0
}

#[cfg(target_arch = "x86")]
#[link_name = "___CxxFrameHandler3@0"]
pub extern "system" fn __cxx_frame_handler3_0() -> i32 {
    0
}
