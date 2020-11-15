use crate::entities::*;
use postgres::Client;

pub trait Create<T> {
    fn create(new : &T, connection : &mut Client) -> bool;
}

impl Create<Good> for Good {
    fn create(new : &Good, connection : &mut Client) -> bool {
        let query = format!(
            "INSERT INTO public.\"{table}\" VALUES (\'{values}) returning *;",
            table = new.table_name,
            values = String::from(&new.good_name) + "\', "
                    + &new.departments_id.to_string() + ", "
                    + &new.categories_id.to_string()
        );

        connection.query(&query[..], &[]).is_ok()
    }
}

impl Create<Category> for Category {
    fn create(new : &Category, connection : &mut Client) -> bool {
        let query = format!(
            "INSERT INTO public.\"{table}\" VALUES \'{values}\'",
            table = new.table_name,
            values = new.category_name
        );

        connection.query(&query[..], &[]).is_ok()
    }
}

impl Create<Department> for Department {
    fn create(new : &Department, connection : &mut Client) -> bool {
        let query = format!(
            "INSERT INTO public.\"{table}\" VALUES \'{values}\'",
            table = new.table_name,
            values = new.department_name
        );

        connection.query(&query[..], &[]).is_ok()
    }
}

impl Create<Order> for Order {
    fn create(new : &Order, connection : &mut Client) -> bool {
        let query = format!(
            "INSERT INTO public.\"{table}\" VALUES \'{values}\'",
            table = new.table_name,
            values = new.orders_name
        );

        connection.query(&query[..], &[]).is_ok()
    }
}