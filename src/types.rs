pub type AllocPointer = u64;

pub enum TrackType {
    Malloc(AllocPointer, usize, String),
    Free(AllocPointer),
    Realloc(AllocPointer, usize, String),
    Calloc(AllocPointer, usize, String)
}