use diesel::{Queryable, Insertable};
use crate::{models::schema::Goods};

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name="Goods"]
pub struct Good {
    pub goods_id       : i32,
    pub good_name      : String,
    pub departments_id : i32,
    pub categories_id  : i32,
}

#[derive(Debug, Queryable)]
pub struct GoodUpdate {
    pub goods_id: i32,
    pub good_name: Option<Option<String>>,
    pub departments_id : i32,
    pub categories_id  : i32,
}

// impl From<crate::cli::GoodUpdate> for GoodUpdate {

//     fn from(cli_upd: crate::cli::GoodUpdate) -> Self { Self {
//         goods_id:        cli_upd.goods_id,
//         good_name:       cli_upd.good_name,
//         departments_id:  cli_upd.departments_id,
//         categories_id:   cli_upd.categories_id,
//     }}
// }