use postgres::Client;

pub struct TakeFromTwo {
    pub connection : Client
}

#[allow(deprecated)]
impl TakeFromTwo {
    pub fn filter_orders_goods(connection : &mut Client, name1 : String, name2 : String) {
        let query = format!(
            "SELECT public.\"Orders\".orders_name, public.\"Goods\".good_name
            FROM public.\"Orders\" join public.\"Orders_Goods\"
            ON public.\"Orders\".orders_id = public.\"Orders_Goods\".orders_id
            join public.\"Goods\"
            ON public.\"Orders_Goods\".goods_id = public.\"Goods\".goods_id
            WHERE orders_name LIKE '%{name1}%' AND good_name LIKE '%{name2}%'",
            name1 = name1,
            name2 = name2
        );

        let t = time::precise_time_ns();

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                let ord_name : String = cur.get("orders_name");
                let goo_name : String = cur.get("good_name");
                println!("{} {}", ord_name, goo_name);
            });

        println!("Time: {} ms", (time::precise_time_ns() - t) / 1000000);
    }

    #[allow(deprecated)]
    pub fn filter_goods_departments(connection : &mut Client, name1 : String, name2 : String) {
        let query = format!(
            "SELECT public.\"Goods\".good_name, public.\"Departments\".department_name
            FROM public.\"Goods\" join public.\"Departments\"
            ON public.\"Goods\".departments_id = public.\"Departments\".departments_id
			WHERE good_name LIKE '%{name1}%' AND department_name LIKE '%{name2}%'",
            name1 = name1,
            name2 = name2
        );

        let t = time::precise_time_ns();

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                let goo_name : String = cur.get("good_name");
                let dep_name : String = cur.get("department_name");
                println!("{} {}", goo_name, dep_name);
            });

        println!("Time: {} ms", (time::precise_time_ns() - t) / 1000000);
    }

    pub fn filter_goods_categories(connection : &mut Client, name1 : String, name2 : String) {
        let query = format!(
            "SELECT public.\"Goods\".good_name, public.\"Categories\".category_name
            FROM public.\"Goods\" join public.\"Categories\"
            ON public.\"Goods\".categories_id = public.\"Categories\".categories_id
			WHERE good_name LIKE '%{name1}%' AND category_name LIKE '%{name2}%'",
            name1 = name1,
            name2 = name2
        );

        let t = time::precise_time_ns();

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                let goo_name : String = cur.get("good_name");
                let cat_name : String = cur.get("category_name");
                println!("{} {}", goo_name, cat_name);
            });
        
        println!("Time: {} ms", (time::precise_time_ns() - t) / 1000000);
    }
}
