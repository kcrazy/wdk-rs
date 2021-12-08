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

const DBG_BUFFER_LEN: usize = 4096;
static mut DBG_BUFFER: [u16; DBG_BUFFER_LEN] = [0; DBG_BUFFER_LEN];

impl Write for Adaptor {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let utf16 = s.encode_utf16();

        unsafe {
            let mut n = 0;
            for str in utf16 {
                if n < DBG_BUFFER_LEN - 1 {
                    DBG_BUFFER[n] = str;
                    n += 1;
                }
            }

            DBG_BUFFER[n] = 0;

            DbgPrint("%ws\0".as_ptr() as _, &DBG_BUFFER);
        };
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    core::fmt::write(&mut Adaptor {}, args).expect("Error occurred while print");
}
