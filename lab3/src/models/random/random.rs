use postgres::{Client};

pub fn random_generate_goods(connection : &mut Client) -> bool {
    let query = include_str!("./sql/random_generate.sql");
    
    connection.batch_execute(&query[..]).is_ok()
}