#![no_std]

use wdk::println;
use wdk_sys::base::{DRIVER_OBJECT, NTSTATUS, STATUS_SUCCESS, UNICODE_STRING};

#[no_mangle]
pub extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _: &UNICODE_STRING) -> NTSTATUS {
    driver.DriverUnload = Some(driver_exit);
    println!("Hello World!");
    STATUS_SUCCESS
}

pub extern "C" fn driver_exit(_driver: *mut DRIVER_OBJECT) {
    println!("Bye bye!");
}
