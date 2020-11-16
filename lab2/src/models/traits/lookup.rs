use crate::entities::*;
use postgres::Client;

pub trait Lookup<T> {
    fn lookup(connection : &mut Client);
}

impl Lookup<Good> for Good {
    fn lookup(connection : &mut Client) {
        let query = format!(
            "SELECT * FROM public.\"Goods\""
        );

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                let goods_id : i32 = cur.get("goods_id");
                let good_name : String = cur.get("good_name");
                let departments_id : i32 = cur.get("departments_id");
                let categories_id : i32 = cur.get("categories_id");

                println!("{} {} {} {}", goods_id, good_name, departments_id, categories_id);
            });
    }
}

impl Lookup<Category> for Category {
    fn lookup(connection : &mut Client) {
        let query = format!(
            "SELECT * FROM public.\"Categories\""
        );

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                let categories_id : i32 = cur.get("categories_id");
                let category_name : String = cur.get("category_name");

                println!("{} {}", categories_id, category_name);
            });
    }
}

impl Lookup<Department> for Department {
    fn lookup(connection : &mut Client) {
        let query = format!(
            "SELECT * FROM public.\"Departments\""
        );

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                let departments_id : i32 = cur.get("departments_id");
                let department_name : String = cur.get("department_name");

                println!("{} {}", departments_id, department_name);
            });
    }
}

impl Lookup<Order> for Order {
    fn lookup(connection : &mut Client) {
        let query = format!(
            "SELECT * FROM public.\"Orders\""
        );

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                let orders_id : i32 = cur.get("orders_id");
                let orders_name : String = cur.get("orders_name");

                println!("{} {}", orders_id, orders_name);
            });
    }
}