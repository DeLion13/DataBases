pub use postgres::Client;
pub use std::io::stdin;
pub use crate::cli::*;

#[allow(unused)]
pub fn run_cli(connection : &mut Client) {
    console_clear();
    loop {
        println!("\nChoose TASK:\n  1 : Task 1\n  2 : Task 2\n  Q : Quit\n");
    
        let mut task_number = String::new();
        stdin().read_line(&mut task_number);
        
        match task_number.trim() {
            "1" => tasks::task1(connection),
            "2" => tasks::task2(connection),
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