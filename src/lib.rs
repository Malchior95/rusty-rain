use std::sync::atomic::AtomicUsize;

pub mod ai;
pub mod config;
pub mod data_helpers;
pub mod math;
pub mod world;
pub mod world_interaction;

pub static FRAME_NUM: AtomicUsize = AtomicUsize::new(0);
