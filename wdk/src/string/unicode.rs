#[macro_export]
macro_rules! unicode_string {
    ($s:expr) => {{
        use wdk_sys::base::USHORT;

        const BUF:&[u16] = $crate::string::encode!($s);
        UNICODE_STRING {
            Length: (BUF.len() * 2) as USHORT,
            MaximumLength: (BUF.len() * 2) as USHORT,
            Buffer: BUF.as_ptr() as _,
        }
    }};
}