#![no_std]
#![no_main]

use wdk::{unicode_string, println};
use wdk_sys::base::{DRIVER_OBJECT, NTSTATUS, STATUS_SUCCESS, UNICODE_STRING};
use wdk_sys::ntoskrnl::DbgPrint;

#[no_mangle]
extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _: &UNICODE_STRING) -> NTSTATUS {
    driver.DriverUnload = Some(driver_exit);
    println!("Hello World!");
    println!("你好，世界！");

    const US: UNICODE_STRING = unicode_string!("你好，世界！");
    unsafe { DbgPrint("Unicode String: %wZ\n\0".as_ptr() as _, &US); }

    STATUS_SUCCESS
}

extern "C" fn driver_exit(_driver: *mut DRIVER_OBJECT) {
    println!("Bye bye!");
}
