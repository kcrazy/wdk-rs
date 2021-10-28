#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

mod allocator;
mod dbg;

use core::panic::PanicInfo;

#[link(name = "vcruntime")]
extern {}

#[link(name = "ucrt")]
extern {}

#[used]
#[no_mangle]
pub static _fltused: i32 = 0;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: allocator::KernelAllocator = allocator::KernelAllocator {};

#[no_mangle]
pub extern "C" fn _start() -> u32 {
    println!("Hello World");
    0
}
