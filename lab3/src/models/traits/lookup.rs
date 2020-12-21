use crate::entities::*;
use diesel::prelude::*;
pub use crate::database::PgConnPool;

pub trait Lookup<T> {
    fn lookup(connection : &mut PgConnPool);
}

impl Lookup<Good> for Good {
    fn lookup(connection : &mut PgConnPool) {
        let tb = crate::schema::Goods::table;
        let all = crate::schema::Goods::all_columns;
        tb.select(all)
            .load::<(i32, String, i32, i32)>(&(connection.get().unwrap()))
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                println!("Id: {} Name: {} Dep.: {} Categ.: {}", cur.0, cur.1, cur.2, cur.3);
            });
    }
}

impl Lookup<Category> for Category {
    fn lookup(connection : &mut PgConnPool) {
        let tb = crate::schema::Categories::table;
        let all = crate::schema::Categories::all_columns;
        tb.select(all)
            .load::<(i32, String)>(&(connection.get().unwrap()))
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                println!("Id: {} Name: {}", cur.0, cur.1);
            });
    }
}

impl Lookup<Department> for Department {
    fn lookup(connection : &mut PgConnPool) {
        let tb = crate::schema::Departments::table;
        let all = crate::schema::Departments::all_columns;
        tb.select(all)
            .load::<(i32, String)>(&(connection.get().unwrap()))
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                println!("Id: {} Name: {}", cur.0, cur.1);
            });
    }
}

impl Lookup<Order> for Order {
    fn lookup(connection : &mut PgConnPool) {
        let tb = crate::schema::Orders::table;
        let all = crate::schema::Orders::all_columns;
        tb.select(all)
            .load::<(i32, String)>(&(connection.get().unwrap()))
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                println!("Id: {} Name: {}", cur.0, cur.1);
            });
    }
}