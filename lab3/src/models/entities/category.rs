use diesel::{Queryable, Insertable};
use crate::{models::schema::Categories};

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name="Categories"]
pub struct Category {
    pub categories_id : i32,
    pub category_name : String,
}

#[derive(Debug, Queryable)]
pub struct CategoryUpdate {
    pub categories_id: i32,
    pub category_name: Option<Option<String>>,
}

// impl From<crate::cli::CategoryUpdate> for CategoryUpdate {

//     fn from(cli_upd: crate::cli::CategoryUpdate) -> Self { Self {
//         categories_id:  cli_upd.categories_id,
//         category_name:  cli_upd.category_name,
//     }}
// }