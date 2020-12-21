use diesel::{Queryable, Insertable};
use crate::{models::schema::Departments};

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name="Departments"]
pub struct Department {
    pub departments_id  : i32,
    pub department_name : String,
}

#[derive(Debug, Queryable)]
pub struct DepartmentUpdate {
    pub departments_id: i32,
    pub department_name: Option<Option<String>>,
}

// impl From<crate::cli::DepartmentUpdate> for DepartmentUpdate {

//     fn from(cli_upd: crate::cli::DepartmentUpdate) -> Self { Self {
//         departments_id:  cli_upd.departments_id,
//         department_name:  cli_upd.department_name,
//     }}
// }