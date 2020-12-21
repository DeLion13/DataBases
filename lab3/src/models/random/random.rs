use diesel::prelude::*;
pub use crate::database::PgConnPool;

#[allow(unused)]
pub fn random_generate_goods(connection : &mut PgConnPool) -> bool {
    let query = include_str!("./sql/random_generate.sql");
    diesel::sql_query(&query[..])
        .execute(&(connection.get().unwrap()));

    true
}