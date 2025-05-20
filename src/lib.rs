use std::sync::atomic::AtomicUsize;

pub mod ai;
pub mod data_helpers;
pub mod math;
pub mod world;

pub static FRAME_NUM: AtomicUsize = AtomicUsize::new(0);
