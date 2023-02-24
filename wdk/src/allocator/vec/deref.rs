use super::Vec;

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        let header = self.header();
        let data = self.data();
        let len = header.len;
        unsafe { core::slice::from_raw_parts(data, len) }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let header = self.header();
        let data = self.data();
        let len = header.len;
        unsafe { core::slice::from_raw_parts_mut(data, len) }
    }
}
