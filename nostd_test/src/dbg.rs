use core::fmt::Arguments;
use winapi::um::debugapi::OutputDebugStringA;
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::dbg::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: Arguments) {
    let str = alloc::format!("{}\n", args);
    unsafe { OutputDebugStringA(str.as_ptr() as _) }
}