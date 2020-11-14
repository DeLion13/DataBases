use postgres::Row;

#[derive(Debug)]
pub struct Department {
    pub departments_id  : i32,
    pub department_name : String,
    pub table_name      : String
}

impl From<Row> for Department {
    fn from(row: Row) -> Self {
        Self {
            departments_id  : row.get("departments_id"),
            department_name : row.get("department_name"),
            table_name      : String::from("Departments")
        }
    }
}

impl<'a> Department {
    pub fn get_columns() -> Vec<&'a str> {
        let mut v = Vec::new();
        v.push("departments_id");
        v.push("department_name");
        v
    }
}