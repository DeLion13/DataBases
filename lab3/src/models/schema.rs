table! {
    use diesel::sql_types::*;

    #[allow(non_snake_case)]
    Categories (categories_id) {
        categories_id -> Int4,
        category_name -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    #[allow(non_snake_case)]
    Departments (departments_id) {
        departments_id -> Int4,
        department_name -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    #[allow(non_snake_case)]
    Goods (goods_id) {
        goods_id -> Int4,
        good_name -> Text,
        departments_id -> Int4,
        categories_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;

    #[allow(non_snake_case)]
    Orders (orders_id) {
        orders_id -> Int4,
        orders_name -> Text,
    }
}


joinable!(Categories -> Goods (categories_id));
joinable!(Departments -> Goods (departments_id));

allow_tables_to_appear_in_same_query!(
    Orders,
    Goods,
    Categories,
    Departments,
);
