#![no_std]

extern crate alloc;

pub mod allocator;
pub mod dbg;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
