pub use std::io::stdin;
pub use crate::cli::*;
pub use crate::database::PgConnPool;

#[allow(unused)]
pub fn run_cli(connection : &mut PgConnPool) {
    console_clear();
    loop {
        println!("\nChoose action:\n  1 : Tables\n  Q : Quit\n");
    
        let mut task_number = String::new();
        stdin().read_line(&mut task_number);
        
        match task_number.trim() {
            "1" => tasks::task1(connection),
            "Q" | "q" => {console_clear(); break},
            _ => {console_clear(); println!("Incorrect number!"); break},
        };
    }
}

pub fn console_clear() {
    print!("{}[2J", 27 as char);
}

#[allow(unused)]
pub fn stop() {
    println!("\n[PRESS ENTER TO CONTINUE]");
    stdin().read_line(&mut String::new());
    console_clear();
}