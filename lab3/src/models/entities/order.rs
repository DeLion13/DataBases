use postgres::Row;

#[derive(Debug)]
pub struct Order {
    pub orders_id   : i32,
    pub orders_name : String,
    pub table_name  : String
}

impl From<Row> for Order {
    fn from(row: Row) -> Self {
        Self {
            orders_id   : row.get("orders_id"),
            orders_name : row.get("orders_name"),
            table_name    : String::from("Orders")
        }
    }
}

impl<'a> Order {
    pub fn get_columns() -> Vec<&'a str> {
        let mut v = Vec::new();
        v.push("orders_id");
        v.push("orders_name");
        v
    }
}