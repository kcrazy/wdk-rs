#![no_std]

mod bind;

pub mod base;

#[cfg(feature = "ntoskrnl")]
pub mod ntoskrnl;

pub use cty::*;

use core::panic::PanicInfo;

#[cfg(not(feature = "usermod"))]
use crate::base::STATUS_ACCESS_VIOLATION;

#[cfg(not(feature = "usermod"))]
use crate::ntoskrnl::KeBugCheck;

/// This function is called on panic.
#[cfg(not(feature = "usermod"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        KeBugCheck(STATUS_ACCESS_VIOLATION as u32);
    }
}

#[cfg(feature = "usermod")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
