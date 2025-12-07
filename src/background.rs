use std::sync::mpsc::*;
use crate::types::TrackType;

pub fn compress_background_thread(recv : Receiver<TrackType>, send : Sender<u8>)  {
    
}

pub fn data_patch_background_thread(recv : Receiver<u8>) {
    
}