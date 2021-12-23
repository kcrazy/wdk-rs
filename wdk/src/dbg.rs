use core::fmt::{Arguments, Write};
use core::mem::size_of;

use alloc::vec::Vec;

use wdk_sys::base::UNICODE_STRING;
use wdk_sys::ntoskrnl::DbgPrint;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::dbg::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

struct Adaptor {}

impl Write for Adaptor {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let utf16: Vec<u16> = s.encode_utf16().collect();

        let s = UNICODE_STRING {
            Length: (utf16.len() * size_of::<u16>()) as u16,
            MaximumLength: (utf16.len() * size_of::<u16>()) as u16,
            Buffer: utf16.as_ptr() as _,
        };

        unsafe {
            DbgPrint("%wZ\0".as_ptr() as _, &s);
        };
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    core::fmt::write(&mut Adaptor {}, args).expect("Error occurred while print");
}
