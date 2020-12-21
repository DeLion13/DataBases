use crate::entities::*;
use diesel::prelude::*;
pub use crate::database::PgConnPool;

pub trait Update<T> {
    fn update(new : &T, connection : &mut PgConnPool) -> bool;
}
#[allow(unused)]
impl Update<Good> for Good {
    fn update(new : &Good, connection : &mut PgConnPool) -> bool {
        let tb = crate::schema::Goods::table;
        let id = crate::schema::Goods::goods_id;
        let name = crate::schema::Goods::good_name;
        let dep_id = crate::schema::Goods::departments_id;
        let cat_id = crate::schema::Goods::categories_id;

        diesel::update(tb.filter(id.eq(new.goods_id)))
            .set((name.eq(new.good_name.clone()), dep_id.eq(new.departments_id), cat_id.eq(new.categories_id)))
            .get_result::<Good>(&(connection.get().unwrap()));

        true
    }
}
#[allow(unused)]
impl Update<Category> for Category {
    fn update(new : &Category, connection : &mut PgConnPool) -> bool {
        let tb = crate::schema::Categories::table;
        let id = crate::schema::Categories::categories_id;
        let name = crate::schema::Categories::category_name;

        diesel::update(tb.filter(id.eq(new.categories_id)))
            .set(name.eq(new.category_name.clone()))
            .get_result::<Category>(&(connection.get().unwrap()));

        true
    }
}
#[allow(unused)]
impl Update<Department> for Department {
    fn update(new : &Department, connection : &mut PgConnPool) -> bool {
        let tb = crate::schema::Departments::table;
        let id = crate::schema::Departments::departments_id;
        let name = crate::schema::Departments::department_name;

        diesel::update(tb.filter(id.eq(new.departments_id)))
            .set(name.eq(new.department_name.clone()))
            .get_result::<Department>(&(connection.get().unwrap()));

        true
    }
}
#[allow(unused)]
impl Update<Order> for Order {
    fn update(new : &Order, connection : &mut PgConnPool) -> bool {
        let tb = crate::schema::Orders::table;
        let id = crate::schema::Orders::orders_id;
        let name = crate::schema::Orders::orders_name;

        diesel::update(tb.filter(id.eq(new.orders_id)))
            .set(name.eq(new.orders_name.clone()))
            .get_result::<Order>(&(connection.get().unwrap()));

        true
    }
}