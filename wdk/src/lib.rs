#![no_std]

pub mod allocator;
pub mod error;
pub mod string;
pub mod sync;


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
#[allow(non_snake_case)]
#[no_mangle]
fn __CxxFrameHandler3() -> i32 {
    0
}
