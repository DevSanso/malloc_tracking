mod utils;
mod hook;
mod background;
mod shm;
mod global;
mod types;
mod init;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mk_get_max_data_total_size() -> libc::c_ulonglong {
    crate::shm::constant::MAX_DATA_TOTAL_SIZE as libc::c_ulonglong
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mk_get_max_data_body_size() -> libc::c_ulonglong {
    crate::shm::constant::MAX_DATA_BODY_SIZE as libc::c_ulonglong
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mk_get_max_data_count() -> libc::c_ulonglong  {
    crate::shm::constant::MAX_DATA_COUNT as libc::c_ulonglong
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_seq_max_count() -> libc::c_ulonglong {
    crate::shm::constant::MAX_SEQ as libc::c_ulonglong
}

