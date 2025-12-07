use libc::{c_void, dlsym, size_t, RTLD_NEXT};
use crate::global::REAL_FN;
use crate::global::real_fn::RealFn;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: size_t) -> *mut c_void {
    let m = REAL_FN.malloc;
    let p = m(size);
    let b = std::backtrace::Backtrace::force_capture();
    
    p
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn calloc(nmemb: size_t, size: size_t) -> *mut c_void {
    let calloc = REAL_FN.calloc;
    let p = calloc(nmemb, size);
    let b = std::backtrace::Backtrace::force_capture();

    p
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    let realloc = REAL_FN.realloc;
    let p = realloc(ptr, size);
    let b = std::backtrace::Backtrace::force_capture();
    p
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
    let f = REAL_FN.free;
    let b = std::backtrace::Backtrace::force_capture();
    
    
    f(ptr as *mut c_void);

}
