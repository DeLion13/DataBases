use crate::models::entities::Category;
use crate::models::traits::*;
use crate::cli::*;
use std::io::stdin;
pub use crate::database::PgConnPool;
use diesel::prelude::*;

pub struct CategoriesController<'a> {
    connection : &'a mut PgConnPool
}

impl<'a> CategoriesController<'a> {
    pub fn new(connection : &'a mut PgConnPool) -> Self {
        Self {connection}
    }

    pub fn get_by_id(self : &mut Self, id : i32) {
        let category = Category::find_by_id(id, &mut self.connection);

        crate::views::display_category_by_id(&category);
    }

    #[allow(unused)]
    pub fn create(self : &mut Self) {
        let mut new_name = String::new();

        println!("[INSERT MODE]\n  Name:");
        stdin().read_line(&mut new_name);

        let tb = crate::schema::Categories::table;
        let all = crate::schema::Categories::all_columns;
        let len = tb.select(all)
            .load::<(i32, String)>(&(self.connection.get().unwrap()))
            .unwrap()
            .last()
            .unwrap()
            .0;

        let category = Category {
            category_name : String::from(new_name.trim()),
            categories_id : len + 1,
        };

        let is_ok = Category::create(&category, &mut self.connection);

        if is_ok {
            crate::views::display_new_category(&category);
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

        let category = Category {
            category_name : String::from(new_name.trim()),
            categories_id : id.trim().parse::<i32>().unwrap_or(1),
        };

        let is_ok = Category::update(&category, &mut self.connection);

        if is_ok {
            crate::views::display_updated_category(&category);
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

        let is_ok = Category::delete_by_id(id_i32, &mut self.connection);

        if is_ok {
            crate::views::display_deleted_category(id_i32, is_ok);
        } else {
            crate::views::display_err();
        }
        stop();
    }

    #[allow(unused)]
    pub fn lookup(self : &mut Self) {
        let is_ok = Category::lookup(&mut self.connection);
        stop();
    }
}