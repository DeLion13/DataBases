use postgres::{NoTls, Client};
use lab2::*;
use crate::traits::*;

fn main() {
    let mut client = Client::connect("postgresql://postgres:1234@localhost:5432/Markets", NoTls).unwrap();
    // let row = client.query("SELECT goods_id, good_name, departments_id, categories_id FROM public.\"Goods\"", &[]).is_ok();

    // println!("{}", row);

    let mut new1 = crate::entities::Good {
        goods_id : 12,
        good_name : String::from("Crocodile"),
        categories_id : 1,
        departments_id : 1,
        table_name : String::from("Goods")
    };

    for row in client.query("SELECT goods_id, good_name, departments_id, categories_id FROM public.\"Goods\"", &[]).unwrap() {
        let new = crate::entities::Good::from(row);    
        println!("found good: {} {} {} {}", new.goods_id, new.good_name, new.departments_id, new.categories_id);
        // new1 = new;

        // println!("{}", crate::entities::Good::create(new1, &mut client));
    }

    println!("{}", crate::entities::Good::delete_by_id(14, &mut client));
    
    // crate::entities::Good::create(new1, &mut client);

    // "SELECT goods_id, good_name, departments_id, categories_id FROM Goods"
}
