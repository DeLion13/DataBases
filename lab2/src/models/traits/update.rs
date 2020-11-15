use crate::entities::*;
use postgres::Client;

pub trait Update<T> {
    fn update(new : &T, connection : &mut Client) -> bool;
}

impl Update<Good> for Good {
    fn update(new : &Good, connection : &mut Client) -> bool {
        let vec = Good::get_columns();

        let query = format!(
            "UPDATE public.\"{table}\" SET {values} WHERE {col_id}={id}",
            table = new.table_name,
            values = vec[0].to_owned() + "=\'" + &new.good_name + "\', "
                    + vec[1] + "=" + &new.categories_id.to_string() + ", "
                    + vec[2] + "=" + &new.departments_id.to_string(),
            col_id = vec[4],
            id = new.goods_id
        );

        connection.batch_execute(&query).is_ok()
    }
}

impl Update<Category> for Category {
    fn update(new : &Category, connection : &mut Client) -> bool {
        let vec = Category::get_columns();

        let query = format!(
            "UPDATE public.\"{table}\" SET {values}\' WHERE {col_id}={id}",
            table = new.table_name,
            values = vec[1].to_owned() + "=\'" + &new.category_name,
            col_id = vec[0],
            id = new.categories_id
        );

        connection.batch_execute(&query).is_ok()
    }
}

impl Update<Department> for Department {
    fn update(new : &Department, connection : &mut Client) -> bool {
        let vec = Department::get_columns();

        let query = format!(
            "UPDATE public.\"{table}\" SET {values}\' WHERE {col_id}={id}",
            table = new.table_name,
            values = vec[1].to_owned() + "=\'" + &new.department_name,
            col_id = vec[0],
            id = new.departments_id
        );

        connection.batch_execute(&query).is_ok()
    }
}

impl Update<Order> for Order {
    fn update(new : &Order, connection : &mut Client) -> bool {
        let vec = Order::get_columns();

        let query = format!(
            "UPDATE public.\"{table}\" SET {values}\' WHERE {col_id}={id}",
            table = new.table_name,
            values = vec[1].to_owned() + "=\'" + &new.orders_name,
            col_id = vec[0],
            id = new.orders_id
        );

        connection.batch_execute(&query).is_ok()
    }
}