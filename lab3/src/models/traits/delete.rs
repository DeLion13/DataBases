use crate::entities::*;
use diesel::prelude::*;
pub use crate::database::PgConnPool;

pub trait Delete {
    fn delete_by_id(index : i32, connection : &mut PgConnPool) -> bool;
}
#[allow(unused)]
impl Delete for Good {
    fn delete_by_id(index : i32, connection : &mut PgConnPool) -> bool {
        let tb = crate::schema::Goods::table;
        let id = crate::schema::Goods::goods_id;
        diesel::delete(tb)
            .filter(id.eq(index))
            .execute(&(connection.get().unwrap()))
            .map(|rows_affected| rows_affected > 0);

        true
    }
}
#[allow(unused)]
impl Delete for Category {
    fn delete_by_id(index : i32, connection : &mut PgConnPool) -> bool {
        let tb = crate::schema::Categories::table;
        let id = crate::schema::Categories::categories_id;
        diesel::delete(tb)
            .filter(id.eq(index))
            .execute(&(connection.get().unwrap()))
            .map(|rows_affected| rows_affected > 0);

        true
    }
}
#[allow(unused)]
impl Delete for Department {
    fn delete_by_id(index : i32, connection : &mut PgConnPool) -> bool {
        let tb = crate::schema::Departments::table;
        let id = crate::schema::Departments::departments_id;
        diesel::delete(tb)
            .filter(id.eq(index))
            .execute(&(connection.get().unwrap()))
            .map(|rows_affected| rows_affected > 0);

        true
    }
}
#[allow(unused)]
impl Delete for Order {
    fn delete_by_id(index : i32, connection : &mut PgConnPool) -> bool {
        let tb = crate::schema::Orders::table;
        let id = crate::schema::Orders::orders_id;
        diesel::delete(tb)
            .filter(id.eq(index))
            .execute(&(connection.get().unwrap()))
            .map(|rows_affected| rows_affected > 0);

        true
    }
}