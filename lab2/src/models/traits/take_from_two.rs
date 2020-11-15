use postgres::Client;

pub struct TakeFromTwo {
    pub connection : Client
}

impl TakeFromTwo {
    pub fn take_from_two(connection : &mut Client, name : String) {
        let query = format!(
            "SELECT public.\"Orders\".orders_name, public.\"Goods\".good_name
            FROM public.\"Orders\" join public.\"Orders_Goods\"
            ON public.\"Orders\".orders_id = public.\"Orders_Goods\".orders_id
            join public.\"Goods\"
            ON public.\"Orders_Goods\".goods_id = public.\"Goods\".goods_id
            WHERE good_name LIKE '%{name}%'",
            name = name
        );

        connection.query(&query[..], &[])
            .unwrap()
            .into_iter()
            .for_each(|cur| {
                let ord_name : String = cur.get("orders_name");
                let goo_name : String = cur.get("good_name");
                println!("{} {}", ord_name, goo_name);
            });
    }
}
