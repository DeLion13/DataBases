use crate::entities::*;
use postgres::Client;

pub trait Delete {
    fn delete_by_id(index : i32, connection : &mut Client) -> bool;
}

impl Delete for Good {
    fn delete_by_id(index : i32, connection : &mut Client) -> bool {
        let query = format!(
            "DELETE FROM public.\"{table}\" WHERE goods_id={values};",
            table = "Goods",
            values = index
        );

        connection.batch_execute(&query).is_ok()
    }
}

impl Delete for Category {
    fn delete_by_id(index : i32, connection : &mut Client) -> bool {
        let query = format!(
            "DELETE FROM public.\"{table}\" WHERE categories_id={values}",
            table = "Categories",
            values = index
        );

        connection.batch_execute(&query).is_ok()
    }
}

impl Delete for Department {
    fn delete_by_id(index : i32, connection : &mut Client) -> bool {
        let query = format!(
            "DELETE FROM public.\"{table}\" WHERE departments_id={values}",
            table = "Departments",
            values = index
        );

        connection.batch_execute(&query).is_ok()
    }
}

impl Delete for Order {
    fn delete_by_id(index : i32, connection : &mut Client) -> bool {
        let query = format!(
            "DELETE FROM public.\"{table}\" WHERE orders_id={values}",
            table = "Orders",
            values = index
        );

        connection.batch_execute(&query).is_ok()
    }
}