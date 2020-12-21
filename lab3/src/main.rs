use std::{process, time::Duration};
pub use lab3::*;
pub use crate::cli::*;
pub use crate::schema::*;

#[allow(unused)]
fn main() {
    use lab3::PgConnManager;

    let mut pool = lab3::PgConnPool::builder()
        .connection_timeout(Duration::new(2, 0))
        .max_size(1)
        .build(PgConnManager::new(include_str!("config.txt")))
        .unwrap_or_else(|err| {
            eprintln!(
                "Failed to establish connection to database, please check the \
                correctness of configuration evironment variables or your \
                nerwork connection: {}",
                err
            );
            process::exit(1);
        });
        
    run_cli(&mut pool);
}