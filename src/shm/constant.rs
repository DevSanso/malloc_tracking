use super::types::{Seq, Size};
pub const MAX_DATA_TOTAL_SIZE : usize = 1024 * 1024 * 10;

pub const MAX_DATA_BODY_SIZE: usize = (MAX_DATA_TOTAL_SIZE / MAX_DATA_COUNT) 
    - size_of::<super::types::Seq>() - size_of::<Size>();

pub const MAX_DATA_COUNT : usize = 100;
pub const MAX_SEQ : usize = 5;

