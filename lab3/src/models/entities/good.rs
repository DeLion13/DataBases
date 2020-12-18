use postgres::Row;

#[derive(Debug)]
pub struct Good {
    pub goods_id       : i32,
    pub good_name      : String,
    pub departments_id : i32,
    pub categories_id  : i32,
    pub table_name     : String
}

impl From<Row> for Good {
    fn from(row: Row) -> Self {
        Self {
            goods_id       : row.get("goods_id"),
            good_name      : row.get("good_name"),
            departments_id : row.get("departments_id"),
            categories_id  : row.get("categories_id"),
            table_name     : String::from("Goods")
        }
    }
}

impl<'a> Good {
    pub fn get_columns() -> Vec<&'a str> {
        let mut v = Vec::new();
        v.push("good_name");
        v.push("departments_id");
        v.push("categories_id");
        v.push("goods_id");
        v
    }
}