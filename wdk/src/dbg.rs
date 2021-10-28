use core::fmt::{Arguments, Write};
use wdk_sys::base::ANSI_STRING;
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
        let ansi_str = ANSI_STRING {
            Length: s.len() as u16,
            MaximumLength: s.len() as u16,
            Buffer: s.as_ptr() as _,
        };
        unsafe { DbgPrint("%Z\0".as_ptr() as _, &ansi_str) };
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    core::fmt::write(&mut Adaptor {}, args).expect("Error occurred while print");
}
