#![no_std]

use wdk::println;
use wdk_sys::base::{DRIVER_OBJECT, NTSTATUS, STATUS_SUCCESS, UNICODE_STRING};

#[no_mangle]
pub extern "system" fn driver_entry(_driver: &mut DRIVER_OBJECT, _: &UNICODE_STRING) -> NTSTATUS {
    println!("Hello World!");
    STATUS_SUCCESS
}
