mod unicode;

pub use const_utf16::{encode, encode_null_terminated};


#[macro_export]
macro_rules! unicode {
    ($s:expr) => {{
        $crate::string::encode_null_terminated!($s)
    }};
}