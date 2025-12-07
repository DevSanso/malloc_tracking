pub mod real_fn;
mod alloc;
mod queue;

use std::sync::{LazyLock, OnceLock};

#[global_allocator]
static GLOBAL_ALLOCATOR: crate::global::alloc::InternalAllocator = crate::global::alloc::InternalAllocator;

pub static REAL_FN : LazyLock<real_fn::RealFn> = LazyLock::new( || unsafe {
    real_fn::RealFn::init()
});

