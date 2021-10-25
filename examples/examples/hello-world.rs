#![no_std]

use wdk::println;

#[no_mangle]
extern "C" fn _DllMainCRTStartup() {
    println!("Hello World!");
}
