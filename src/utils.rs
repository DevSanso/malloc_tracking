


pub fn get_os_thread_id() -> libc::pid_t {
    unsafe { libc::gettid() }
}