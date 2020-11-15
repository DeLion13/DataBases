pub mod cli;

mod categories_controller;
mod departments_controller;
mod goods_controller;
mod orders_controller;

pub use categories_controller::*;
pub use departments_controller::*;
pub use goods_controller::*;
pub use orders_controller::*;