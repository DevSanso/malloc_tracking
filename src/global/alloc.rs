use std::alloc::{GlobalAlloc, Layout, System};
use std::ffi::c_void;
use std::ptr;
use std::sync::{atomic, Once, OnceLock};
use libc::{dlsym, size_t, RTLD_NEXT};

static RAW_MALLOC : OnceLock<unsafe extern "C" fn(size_t) -> *mut c_void> = OnceLock::new();
static RAW_FREE : OnceLock<unsafe extern "C" fn(ptr: *mut c_void)> = OnceLock::new();
pub(super) struct InternalAllocator;

unsafe impl GlobalAlloc for InternalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let malloc = RAW_MALLOC.get_or_init(|| {std::mem::transmute(dlsym(RTLD_NEXT, "malloc\0".as_ptr() as *const i8))});
        let ptr = malloc(layout.size());
        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        let free = RAW_FREE.get_or_init(|| {std::mem::transmute(dlsym(RTLD_NEXT, "free\0".as_ptr() as *const i8))});
        free(ptr as *mut c_void);
    }
}

