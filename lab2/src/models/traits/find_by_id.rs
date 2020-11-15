use crate::entities::*;
use postgres::Client;

pub trait FindById<T> {
    fn find_by_id(id : i32, connection : &mut Client) -> T;
}

impl FindById<Good> for Good {
    fn find_by_id(id : i32, connection : &mut Client) -> Good {
        let vec = Good::get_columns();

        let query = format!(
            "SELECT * FROM public.\"Goods\" WHERE {col_id}={id}",
            col_id = vec[3],
            id = id
        );

        let new = Good::from(connection.query_one(&query[..], &[]).unwrap());
        new
    }
}

impl FindById<Category> for Category {
    fn find_by_id(id : i32, connection : &mut Client) -> Category {
        let vec = Category::get_columns();

        let query = format!(
            "SELECT * FROM public.\"Categories\" WHERE {col_id}={id}",
            col_id = vec[0],
            id = id
        );  

        let new = Category::from(connection.query_one(&query[..], &[]).unwrap());
        new
    }
}

impl FindById<Department> for Department {
    fn find_by_id(id : i32, connection : &mut Client) -> Department {
        let vec = Department::get_columns();

        let query = format!(
            "SELECT * FROM public.\"Departments\" WHERE {col_id}={id}",
            col_id = vec[0],
            id = id
        );

        let new = Department::from(connection.query_one(&query[..], &[]).unwrap());
        new
    }
}

impl FindById<Order> for Order {
    fn find_by_id(id : i32, connection : &mut Client) -> Order {
        let vec = Order::get_columns();

        let query = format!(
            "SELECT * FROM public.\"Orders\" WHERE {col_id}={id}",
            col_id = vec[0],
            id = id
        );

        let new = Order::from(connection.query_one(&query[..], &[]).unwrap());
        new
    }
}