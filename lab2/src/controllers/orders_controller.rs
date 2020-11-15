use crate::models::entities::Order;
use crate::models::traits::*;
use postgres::Client;

pub struct OrdersController {
    connection : Client
}

impl OrdersController {
    pub fn new(connection : Client) -> Self {
        Self {connection}
    }

    pub fn get_by_id(self : &mut Self, id : i32) {
        let order = Order::find_by_id(id, &mut self.connection);

        crate::views::display_order_by_id(&order);
    }

    pub fn get_by_name(self : &mut Self, name : String) {
        let order = Order::find_by_name(name, &mut self.connection);

        order
            .iter()
            .for_each(|order| crate::views::display_order_by_id(&order));
    }

    pub fn create(self : &mut Self, order: &Order) {
        let is_ok = Order::create(order, &mut self.connection);

        if is_ok {
            crate::views::display_new_order(order);
        } else {
            crate::views::display_err();
        }
    }

    pub fn update(self : &mut Self, order : &Order) {
        let is_ok = Order::update(order, &mut self.connection);

        if is_ok {
            crate::views::display_updated_order(order);
        } else {
            crate::views::display_err();
        }
    }

    pub fn delete(self : &mut Self, id: i32) {
        let is_ok = Order::delete_by_id(id, &mut self.connection);

        if is_ok {
            crate::views::display_deleted_order(id, is_ok);
        } else {
            crate::views::display_err();
        }
    }
}