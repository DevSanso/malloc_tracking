use std::ffi::{c_void, CString};
use libc::{dlsym, size_t, RTLD_NEXT};

pub struct RealFn {
    pub malloc : unsafe extern "C" fn(size_t) -> *mut c_void,
    pub calloc : unsafe extern "C" fn(size_t, size_t) -> *mut c_void,
    pub realloc : unsafe extern "C" fn(ptr: *mut c_void, size_t) -> *mut c_void,
    pub free : unsafe extern "C" fn(ptr: *mut c_void),
}
impl RealFn {
    pub unsafe fn init() -> RealFn{
        let m = std::mem::transmute(dlsym(RTLD_NEXT, "malloc\0".as_ptr() as *const i8));
        let c = std::mem::transmute(dlsym(RTLD_NEXT, "calloc\0".as_ptr() as *const i8));
        let r = std::mem::transmute(dlsym(RTLD_NEXT, "realloc\0".as_ptr() as *const i8));
        let f = std::mem::transmute(dlsym(RTLD_NEXT, "free\0".as_ptr() as *const i8));

        RealFn {
            malloc : m,
            calloc : c,
            realloc : r,
            free : f,
        }
    }
}