mod goods_view;
mod departments_view;
mod orders_view;
mod categories_view;

pub use goods_view::*;
pub use departments_view::*;
pub use orders_view::*;
pub use categories_view::*;

pub fn display_err() {
    eprintln!("Something wrong!");
}
