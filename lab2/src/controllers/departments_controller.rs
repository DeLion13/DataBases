use crate::models::entities::Department;
use crate::models::traits::*;
use crate::cli::*;
use postgres::Client;
use std::io::stdin;

pub struct DepartmentsController<'a> {
    connection : &'a mut Client
}

impl<'a> DepartmentsController<'a> {
    pub fn new(connection : &'a mut Client) -> Self {
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

    #[allow(unused)]
    pub fn create(self : &mut Self) {
        let mut new_name = String::new();

        println!("[INSERT MODE]\n  Name:");
        stdin().read_line(&mut new_name);

        let department = Department {
            department_name : String::from(new_name.trim()),
            departments_id : 0,
            table_name : String::from("Department")
        };

        let is_ok = Department::create(&department, &mut self.connection);

        if is_ok {
            crate::views::display_new_department(&department);
        } else {
            crate::views::display_err();
        }
        stop();
    }

    #[allow(unused)]
    pub fn update(self : &mut Self) {
        let mut id = String::new();
        let mut new_name = String::new();

        println!("[EDITING MODE]\n  Id:");
        stdin().read_line(&mut id);
        println!("  Name:");
        stdin().read_line(&mut new_name);

        let department = Department {
            department_name : String::from(new_name.trim()),
            departments_id : id.trim().parse::<i32>().unwrap_or(1),
            table_name : String::from("Department")
        };
        
        let is_ok = Department::update(&department, &mut self.connection);

        if is_ok {
            crate::views::display_updated_department(&department);
        } else {
            crate::views::display_err();
        }
        stop();
    }

    #[allow(unused)]
    pub fn delete(self : &mut Self) {
        let mut id = String::new();
        
        println!("[DELETING MODE]\n  Id: ");
        stdin().read_line(&mut id);

        let id_i32 = id.trim().parse::<i32>().unwrap_or(1);

        let is_ok = Department::delete_by_id(id_i32, &mut self.connection);

        if is_ok {
            crate::views::display_deleted_department(id_i32, is_ok);
        } else {
            crate::views::display_err();
        }
        stop();
    }

    #[allow(unused)]
    pub fn lookup(self : &mut Self) {
        let is_ok = Department::lookup(&mut self.connection);
        stop();
    }
}