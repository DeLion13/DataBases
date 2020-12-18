use postgres::Row;

#[derive(Debug)]
pub struct Category {
    pub categories_id : i32,
    pub category_name : String,
    pub table_name    : String
}

impl From<Row> for Category {
    fn from(row: Row) -> Self { 
        Self {
            categories_id : row.get("categories_id"),
            category_name : row.get("category_name"),
            table_name    : String::from("Categories")
        }
    }
}

impl<'a> Category {
    pub fn get_columns() -> Vec<&'a str> {
        let mut v = Vec::new();
        v.push("categories_id");
        v.push("category_name");
        v
    }
}