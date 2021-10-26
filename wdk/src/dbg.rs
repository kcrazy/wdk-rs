use core::fmt::{Arguments, Write};
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
        unsafe { DbgPrint(s.as_ptr() as _) };
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    core::fmt::write(&mut Adaptor {}, args).expect("Error occurred while print");
}
