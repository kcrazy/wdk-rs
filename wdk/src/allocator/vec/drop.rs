use super::Header;
use super::Vec;

use wdk_sys::ntoskrnl::ExFreePoolWithTag;

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {
            #[allow(clippy::cast_ptr_alignment)]
            let Header {
                len,
                cap: _,
                tag,
                pool_type: _,
            } = core::ptr::read(self.buf.as_ptr().cast::<Header>());

            core::ptr::drop_in_place(core::ptr::slice_from_raw_parts_mut(self.data(), len));
            ExFreePoolWithTag(self.buf.as_ptr() as _, tag);
        };
    }
}
