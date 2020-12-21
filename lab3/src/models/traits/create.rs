use crate::entities::*;
use diesel::prelude::*;
use crate::{database::PgConnPool};

pub trait Create<T> {
    fn create(new : &T, connection : &mut PgConnPool) -> bool;
}
#[allow(unused)]
impl Create<Good> for Good {
    fn create(new : &Good, connection : &mut PgConnPool) -> bool {
        diesel::insert_into(crate::schema::Goods::table)
            .values(new)
            .get_result::<Good>(&(connection.get().unwrap()));

        true
    }
}
#[allow(unused)]
impl Create<Category> for Category {
    fn create(new : &Category, connection : &mut PgConnPool) -> bool {
        diesel::insert_into(crate::schema::Categories::table)
            .values(new)
            .get_result::<Category>(&(connection.get().unwrap()));

        true
    }
}
#[allow(unused)]
impl Create<Department> for Department {
    fn create(new : &Department, connection : &mut PgConnPool) -> bool {
        diesel::insert_into(crate::schema::Departments::table)
            .values(new)
            .get_result::<Department>(&(connection.get().unwrap()));

        true
    }
}
#[allow(unused)]
impl Create<Order> for Order {
    fn create(new : &Order, connection : &mut PgConnPool) -> bool {
        diesel::insert_into(crate::schema::Orders::table)
            .values(new)
            .get_result::<Order>(&(connection.get().unwrap()));

        true
    }
}