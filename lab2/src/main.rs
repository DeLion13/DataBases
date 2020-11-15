pub use postgres::{NoTls, Client};
pub use lab2::*;
pub use crate::traits::*;
pub use crate::entities::*;
pub use crate::cli::*;
pub use std::io::stdin;

#[allow(unused)]
fn main() {
    let mut client = Client::connect(include_str!("config.txt"), NoTls).unwrap();
    run_cli(&mut client);
}