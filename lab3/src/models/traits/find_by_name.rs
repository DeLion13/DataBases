use crate::entities::*;
use postgres::Client;

pub trait FindByName<T> {
    fn find_by_name(name : String, connection : &mut Client) -> Vec<T>;
}

impl FindByName<Good> for Good {
    fn find_by_name(name : String, connection : &mut Client) -> Vec<Good> {
        let vec = Good::get_columns();
        let mut res : Vec<Good> = Vec::new();

        let query = format!(
            "SELECT * FROM public.\"Goods\" WHERE {col} LIKE '%{string}%'",
            col = vec[0],
            string = name
        );

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| res.push(Good::from(cur)));

        res
    }
}

impl FindByName<Category> for Category {
    fn find_by_name(name : String, connection : &mut Client) -> Vec<Category> {
        let vec = Category::get_columns();
        let mut res : Vec<Category> = Vec::new();

        let query = format!(
            "SELECT * FROM public.\"Categories\" WHERE {col} LIKE '%{string}%'",
            col = vec[1],
            string = name
        );

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| res.push(Category::from(cur)));

        res
    }
}

impl FindByName<Department> for Department {
    fn find_by_name(name : String, connection : &mut Client) -> Vec<Department> {
        let vec = Department::get_columns();
        let mut res : Vec<Department> = Vec::new();

        let query = format!(
            "SELECT * FROM public.\"Departments\" WHERE {col} LIKE '%{string}%'",
            col = vec[1],
            string = name
        );

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| res.push(Department::from(cur)));

        res
    }
}

impl FindByName<Order> for Order {
    fn find_by_name(name : String, connection : &mut Client) -> Vec<Order> {
        let vec = Order::get_columns();
        let mut res : Vec<Order> = Vec::new();

        let query = format!(
            "SELECT * FROM public.\"Orders\" WHERE {col} LIKE '%{string}%'",
            col = vec[1],
            string = name
        );

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| res.push(Order::from(cur)));

        res
    }
}