pub use postgres::Client;
pub use std::io::stdin;
pub use crate::cli::*;
pub use crate::traits::*;
pub use crate::entities::*;
pub use crate::controllers::*;

extern crate time;

#[allow(unused)]
pub fn task1(connection : &mut Client) {
    console_clear();
    loop {
        println!("Choose TABLE:\n  1 : Goods\n  2 : Categories\n  3 : Departments\n  4 : Orders\n  Q : Quit\n");

        let mut table_number = String::new();
        stdin().read_line(&mut table_number);
        match table_number.trim() {
            "1" => task1_goods_menu(connection),
            "2" => task1_categories_menu(connection),
            "3" => task1_departments_menu(connection),
            "4" => task1_orders_menu(connection),
            "Q" => {console_clear(); break},
            _ => println!("Incorrect number!"),
        };
    }
}

#[allow(unused)]
pub fn task1_goods_menu(connection : &mut Client) {

    let mut gc : GoodsController = GoodsController::new(connection);

    loop {
        console_clear();
        println!("Choose MODE:\n  1 : Create\n  2 : Update\n  3 : Delete\n  4 : Generate\n  5 : Lookup\n  Q : Quit\n");

        let mut mode_number = String::new();
        stdin().read_line(&mut mode_number);

        match mode_number.trim() {
            "1" => gc.create(),
            "2" => gc.update(),
            "3" => gc.delete(),
            "4" => gc.random(),
            "5" => println!("Lookup"),
            "Q" => {console_clear(); break},
            _ => println!("Incorrect number!"),
        };
    }
}

#[allow(unused)]
pub fn task1_categories_menu(connection : &mut Client) {

    let mut cc : CategoriesController = CategoriesController::new(connection);

    loop {
        console_clear();
        println!("Choose MODE:\n  1 : Create\n  2 : Update\n  3 : Delete\n  4 : Lookup\n  Q : Quit\n");

        let mut mode_number = String::new();
        stdin().read_line(&mut mode_number);

        match mode_number.trim() {
            "1" => cc.create(),
            "2" => cc.update(),
            "3" => cc.delete(),
            "4" => println!("Lookup"),
            "Q" => {console_clear(); break},
            _ => println!("Incorrect number!"),
        };
    }
}

#[allow(unused)]
pub fn task1_departments_menu(connection : &mut Client) {

    let mut dc : DepartmentsController = DepartmentsController::new(connection);

    loop {
        console_clear();
        println!("Choose MODE:\n  1 : Create\n  2 : Update\n  3 : Delete\n  4 : Lookup\n  Q : Quit\n");

        let mut mode_number = String::new();
        stdin().read_line(&mut mode_number);

        match mode_number.trim() {
            "1" => dc.create(),
            "2" => dc.update(),
            "3" => dc.delete(),
            "4" => println!("Lookup"),
            "Q" => {console_clear(); break},
            _ => println!("Incorrect number!"),
        };
    }
}

#[allow(unused)]
pub fn task1_orders_menu(connection : &mut Client) {

    let mut oc : OrdersController = OrdersController::new(connection);

    loop {
        console_clear();
        println!("Choose MODE:\n  1 : Create\n  2 : Update\n  3 : Delete\n  4 : Lookup\n  Q : Quit\n");

        let mut mode_number = String::new();
        stdin().read_line(&mut mode_number);

        match mode_number.trim() {
            "1" => oc.create(),
            "2" => oc.update(),
            "3" => oc.delete(),
            "4" => println!("Lookup"),
            "Q" => {console_clear(); break},
            _ => println!("Incorrect number!"),
        };
    }
}

#[allow(unused)]
pub fn task2(connection : &mut Client) {

    let mut gc : GoodsController = GoodsController::new(connection);

    loop {
        console_clear();
        println!("Choose TABLE:\n  1 : Goods with Orders\n  2 : Goods with Categories\n  3 : Goods with Departments\n  Q : Quit\n");

        let mut mode_number = String::new();
        stdin().read_line(&mut mode_number);

        match mode_number.trim() {
            "1" => {
                gc.filter_goods_orders();
                stop();
            },
            "2" => {
                gc.filter_goods_categories();
                stop();
            },
            "3" => {
                gc.filter_goods_departments();
                stop();
            },
            "Q" => {console_clear(); break},
            _ => println!("Incorrect number!"),
        };
    }
}