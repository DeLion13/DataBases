use crate::models::entities::Good;
use crate::models::traits::*;
use crate::cli::*;
pub use crate::database::PgConnPool;
use std::io::stdin;
use diesel::prelude::*;

pub struct GoodsController<'a> {
    connection : &'a mut PgConnPool
}

impl<'a> GoodsController<'a> {
    pub fn new(connection : &'a mut PgConnPool) -> Self {
        Self {connection}
    }

    pub fn get_by_id(self : &mut Self, id : i32) {
        let good = Good::find_by_id(id, self.connection);

        crate::views::display_good_by_id(&good);
    }

    #[allow(unused)]
    pub fn create(self : &mut Self) {
        let mut new_name = String::new();
        let mut dep = String::new();
        let mut cat = String::new();

        println!("[INSERT MODE]\n  Name:");
        stdin().read_line(&mut new_name);
        println!("  Department id:");
        stdin().read_line(&mut dep);
        println!("  Category id:");
        stdin().read_line(&mut cat);

        let tb = crate::schema::Goods::table;
        let all = crate::schema::Goods::all_columns;
        let len = tb.select(all)
            .load::<(i32, String, i32, i32)>(&(self.connection.get().unwrap()))
            .unwrap()
            .last()
            .unwrap()
            .0;


        let good = Good {
            good_name : String::from(new_name.trim()),
            categories_id : cat.trim().parse::<i32>().unwrap_or(1),
            departments_id : dep.trim().parse::<i32>().unwrap_or(1),
            goods_id : len + 1,
        };

        let is_ok = Good::create(&good, &mut self.connection);

        if is_ok {
            crate::views::display_new_good(&good);
        } else {
            crate::views::display_err();
        }

        stop();
    }

    #[allow(unused)]
    pub fn update(self : &mut Self) {
        let mut id = String::new();
        let mut new_name = String::new();
        let mut new_dep = String::new();
        let mut new_cat = String::new();

        println!("[EDITING MODE]\n  Id:");
        stdin().read_line(&mut id);
        println!("  Name:");
        stdin().read_line(&mut new_name);
        println!("  Department id:");
        stdin().read_line(&mut new_dep);
        println!("  Category id:");
        stdin().read_line(&mut new_cat);

        let good = Good {
            good_name : String::from(new_name.trim()),
            categories_id : new_cat.trim().parse::<i32>().unwrap_or(1),
            departments_id : new_dep.trim().parse::<i32>().unwrap_or(1),
            goods_id : id.trim().parse::<i32>().unwrap_or(1),
        };

        let is_ok = Good::update(&good, &mut self.connection);

        if is_ok {
            crate::views::display_updated_good(&good);
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

        let is_ok = Good::delete_by_id(id_i32, &mut self.connection);

        if is_ok {
            crate::views::display_deleted_good(id_i32, is_ok);
        } else {
            crate::views::display_err();
        }

        stop();
    }

    #[allow(unused)]
    pub fn random(self : &mut Self) {
        let is_ok = crate::random::random::random_generate_goods(&mut self.connection);
        
        if !is_ok {
            crate::views::display_err();
        } else {
            println!("Generated!");
        }

        stop();
    }

    #[allow(unused)]
    pub fn lookup(self : &mut Self) {
        let is_ok = Good::lookup(&mut self.connection);
        stop();
    }
}