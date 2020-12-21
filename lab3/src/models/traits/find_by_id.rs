use crate::entities::*;
use diesel::prelude::*;
pub use crate::database::PgConnPool;

pub trait FindById<T> {
    fn find_by_id(id : i32, connection : &mut PgConnPool) -> T;
}

impl FindById<Good> for Good {
    fn find_by_id(id : i32, connection : &mut PgConnPool) -> Good {
        let tb = crate::schema::Goods::table;
        let fid = crate::schema::Goods::goods_id;
        tb.filter(fid.eq(id))
            .get_result(&(connection.get().unwrap()))
            .unwrap()
    }
}

impl FindById<Category> for Category {
    fn find_by_id(id : i32, connection : &mut PgConnPool) -> Category {
        let tb = crate::schema::Categories::table;
        let fid = crate::schema::Categories::categories_id;
        tb.filter(fid.eq(id))
            .get_result(&(connection.get().unwrap()))
            .unwrap()
    }
}

impl FindById<Department> for Department {
    fn find_by_id(id : i32, connection : &mut PgConnPool) -> Department {
        let tb = crate::schema::Departments::table;
        let fid = crate::schema::Departments::departments_id;
        tb.filter(fid.eq(id))
            .get_result(&(connection.get().unwrap()))
            .unwrap()
    }
}

impl FindById<Order> for Order {
    fn find_by_id(id : i32, connection : &mut PgConnPool) -> Order {
        let tb = crate::schema::Orders::table;
        let fid = crate::schema::Orders::orders_id;
        tb.filter(fid.eq(id))
            .get_result(&(connection.get().unwrap()))
            .unwrap()
    }
}