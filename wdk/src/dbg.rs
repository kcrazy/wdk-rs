use core::fmt::{self, Arguments, Write};

use wdk_sys::ntoskrnl::DbgPrint;

use crate::string::UnicodeString;

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
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let us = UnicodeString::from_str(s).map_err(|_err| fmt::Error)?;
        unsafe {
            DbgPrint("%wZ\0".as_ptr() as _, &us.to_unicode_string());
        };
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    core::fmt::write(&mut Adaptor {}, args).expect("Error occurred while print");
}
