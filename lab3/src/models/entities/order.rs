use diesel::{Queryable, Insertable};
use crate::{models::schema::Orders};


#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name="Orders"]
pub struct Order {
    pub orders_id   : i32,
    pub orders_name : String,
}

#[derive(Debug, Queryable)]
pub struct OrderUpdate {
    pub orders_id: i32,
    pub orders_name: Option<Option<String>>,
}

// impl From<crate::cli::OrderUpdate> for OrderUpdate {

//     fn from(cli_upd: crate::cli::OrderUpdate) -> Self { Self {
//         orders_id:    cli_upd.orders_id,
//         orders_name:  cli_upd.orders_name,
//     }}
// }