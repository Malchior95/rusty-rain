use crate::{
    math::Pos,
    world::{actions::BasicAction, inventory::Inventory},
};

use super::Worker;

//use #![feature(macro_metavar_expr_concat)] once that becomes stable, rather than paste
use paste::paste;
macro_rules! worker_impl {
    ($name:ident, $type:ty) => {
        paste! {
        impl Worker {
            pub fn $name(&self) -> &$type {
                match self {
                    Worker::Idle(w) => &w.$name,
                    Worker::Supplying(w) => &w.$name,
                    Worker::Storing(w) => &w.$name,
                    Worker::Gathering(w) => &w.$name,
                    Worker::Returning(w) => &w.$name,
                    Worker::TakingBreak(w) => &w.$name,
                    Worker::Producing(w) => &w.$name,
                    Worker::Lost(w) => &w.$name,
                    Worker::SupplyingBuildZone(w) => &w.$name,
                    Worker::Building(w) => &w.$name,
                }
            }

            pub fn [<$name _mut>](&mut self) -> &mut $type {
                match self {
                    Worker::Idle(w) => &mut w.$name,
                    Worker::Supplying(w) => &mut w.$name,
                    Worker::Storing(w) => &mut w.$name,
                    Worker::Gathering(w) => &mut w.$name,
                    Worker::Returning(w) => &mut w.$name,
                    Worker::TakingBreak(w) => &mut w.$name,
                    Worker::Producing(w) => &mut w.$name,
                    Worker::Lost(w) => &mut w.$name,
                    Worker::SupplyingBuildZone(w) => &mut w.$name,
                    Worker::Building(w) => &mut w.$name,
                }
            }
        }
        }
    };
}

worker_impl!(pos, Pos);
worker_impl!(inventory, Inventory);
worker_impl!(name, String);
worker_impl!(break_progress, BasicAction);
worker_impl!(exhausted, bool);
