use crate::models::entities::Department;
use crate::models::traits::*;
use postgres::Client;

pub struct DepartmentsController {
    connection : Client
}

impl DepartmentsController {
    pub fn new(connection : Client) -> Self {
        Self {connection}
    }

    pub fn get_by_id(self : &mut Self, id : i32) {
        let department = Department::find_by_id(id, &mut self.connection);

        crate::views::display_department_by_id(&department);
    }

    pub fn get_by_name(self : &mut Self, name : String) {
        let department = Department::find_by_name(name, &mut self.connection);

        department
            .iter()
            .for_each(|department| crate::views::display_department_by_id(&department));
    }

    pub fn create(self : &mut Self, department: &Department) {
        let is_ok = Department::create(department, &mut self.connection);

        if is_ok {
            crate::views::display_new_department(department);
        } else {
            crate::views::display_err();
        }
    }

    pub fn update(self : &mut Self, department : &Department) {
        let is_ok = Department::update(department, &mut self.connection);

        if is_ok {
            crate::views::display_updated_department(department);
        } else {
            crate::views::display_err();
        }
    }

    pub fn delete(self : &mut Self, id: i32) {
        let is_ok = Department::delete_by_id(id, &mut self.connection);

        if is_ok {
            crate::views::display_deleted_department(id, is_ok);
        } else {
            crate::views::display_err();
        }
    }
}