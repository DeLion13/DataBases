pub use std::io::stdin;
pub use crate::cli::*;
pub use crate::traits::*;
pub use crate::entities::*;
pub use crate::controllers::*;


#[allow(unused)]
pub fn task1(connection : &mut PgConnPool) {
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
            "Q" | "q" => {console_clear(); break},
            _ => println!("Incorrect number!"),
        };
    }
}

#[allow(unused)]
pub fn task1_goods_menu(connection : &mut PgConnPool) {

    let mut gc : GoodsController = GoodsController::new(connection);

    loop {
        console_clear();
        println!("Choose MODE:\n  1 : Create\n  2 : Update\n  3 : Delete\n  4 : Lookup\n  Q : Quit\n");

        let mut mode_number = String::new();
        stdin().read_line(&mut mode_number);

        match mode_number.trim() {
            "1" => gc.create(),
            "2" => gc.update(),
            "3" => gc.delete(),
            "4" => gc.lookup(),
            "Q" | "q" => {console_clear(); break},
            _ => println!("Incorrect number!"),
        };
    }
}

#[allow(unused)]
pub fn task1_categories_menu(connection : &mut PgConnPool) {

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
            "4" => cc.lookup(),
            "Q" | "q" => {console_clear(); break},
            _ => println!("Incorrect number!"),
        };
    }
}

#[allow(unused)]
pub fn task1_departments_menu(connection : &mut PgConnPool) {

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
            "4" => dc.lookup(),
            "Q" | "q" => {console_clear(); break},
            _ => println!("Incorrect number!"),
        };
    }
}

#[allow(unused)]
pub fn task1_orders_menu(connection : &mut PgConnPool) {

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
            "4" => oc.lookup(),
            "Q" | "q" => {console_clear(); break},
            _ => println!("Incorrect number!"),
        };
    }
}