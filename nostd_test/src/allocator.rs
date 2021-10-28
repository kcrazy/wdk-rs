use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use core::ptr::{null, null_mut};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::memoryapi::{VirtualAlloc, VirtualFree};
use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE};

#[alloc_error_handler]
fn alloc_error(_: Layout) -> ! {
    panic!("alloc_error")
}

unsafe fn map(size: usize, perms: u32, commit: bool) -> Option<*mut u8> {
    let typ = MEM_RESERVE | if commit { MEM_COMMIT } else { 0 };

    let ptr = VirtualAlloc(ptr::null_mut(), size, typ, perms) as *mut u8;

    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}

unsafe fn unmap(ptr: *mut u8, _size: usize) {
    // NOTE: VirtualFree, when unmapping memory (as opposed to decommitting it), can only operate
    // on an entire region previously mapped with VirtualAlloc. As a result, 'ptr' must have been
    // previously returned by VirtualAlloc, and no length is needed since it is known by the kernel
    // (VirtualFree /requires/ that if the third argument is MEM_RELEASE, the second is 0).
    let ret = VirtualFree(ptr as *mut _, 0, MEM_RELEASE);
    assert_ne!(
        ret,
        0,
        "Call to VirtualFree({:?}, 0, MEM_RELEASE) failed with error code {}.",
        ptr,
        GetLastError()
    );
}

pub struct KernelAllocator {}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = map(layout.size(), 0, true);

        if let Some(p) = ptr {
            p as _
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unmap(ptr as _, 0)
    }
}
