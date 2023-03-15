#![no_std]

mod bind;

pub mod base;

#[cfg(feature = "ntoskrnl")]
pub mod ntoskrnl;

pub use cty::*;

use core::panic::PanicInfo;

use crate::base::STATUS_ACCESS_VIOLATION;
use crate::ntoskrnl::KeBugCheck;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        KeBugCheck(STATUS_ACCESS_VIOLATION as u32);
    }
}
