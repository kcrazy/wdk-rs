use wdk_sys::base::LIST_ENTRY;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ListEntry {
    pub flink: *mut ListEntry,
    pub blink: *mut ListEntry,
}

impl ListEntry {
    pub fn from(list: LIST_ENTRY) -> Self {
        ListEntry {
            flink: list.Flink as _,
            blink: list.Blink as _,
        }
    }
}
