use super::constant::*;
use super::types::{Seq, Size};
pub struct ShmLayoutHeader {
    pid : u64,
    proc_name : [u8;256],
    max_seq_count : u16,
    max_data_size : Size,
}
pub struct ShmLayoutDataBody {
    size : Size,
    data : [u8; MAX_DATA_BODY_SIZE],
}

pub struct ShmLayoutData {
    seq : Seq,
    count : u16,
    body : [ShmLayoutDataBody; MAX_DATA_COUNT],
}

pub struct ShmLayout {
    header : ShmLayoutHeader,
    data : [ShmLayoutData; MAX_SEQ]
}

pub struct ShmLayoutFrame {
    header : [u8; std::mem::size_of::<ShmLayoutHeader>()],
    data   : [u8; std::mem::size_of::<ShmLayoutData>() * MAX_SEQ],
}
