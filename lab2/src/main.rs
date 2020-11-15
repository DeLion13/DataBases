pub use postgres::{NoTls, Client};
pub use crate::traits::*;
pub use lab2::*;

#[allow(unused)]

fn main() {
    let mut client = Client::connect(include_str!("config.txt"), NoTls).unwrap();
    // let row = client.query("SELECT goods_id, good_name, departments_id, categories_id FROM public.\"Goods\"", &[]).is_ok();

    // println!("{}", row);
    let mut new1 = crate::entities::Good {
        goods_id : 12,
        good_name : String::from("Crocodile"),
        categories_id : 1,
        departments_id : 1,
        table_name : String::from("Goods")
    };

    let query = "SELECT goods_id, good_name, departments_id, categories_id FROM public.\"Goods\"";

    for row in client.query(query, &[]).unwrap() {
        let new = crate::entities::Good::from(row);    
        println!("found good: {} {} {} {}", new.goods_id, new.good_name, new.departments_id, new.categories_id);
        // new1 = new;

        // println!("{}", crate::entities::Good::create(new1, &mut client));
    }

    // println!("{}", crate::entities::Good::delete_by_id(14, &mut client));

    // println!("{}", crate::random::random_generate_goods(&mut client));
    
    // crate::entities::Good::create(new1, &mut client);

    // "SELECT goods_id, good_name, departments_id, categories_id FROM Goods"
}
