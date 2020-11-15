use crate::models::entities::Good;
use crate::models::traits::*;
use postgres::Client;

pub struct GoodsController {
    connection : Client
}

impl GoodsController {
    pub fn new(connection : Client) -> Self {
        Self {connection}
    }

    pub fn get_by_id(self : &mut Self, id : i32) {
        let good = Good::find_by_id(id, &mut self.connection);

        crate::views::display_good_by_id(&good);
    }

    pub fn get_by_name(self : &mut Self, name : String) {
        let good = Good::find_by_name(name, &mut self.connection);

        good
            .iter()
            .for_each(|good| crate::views::display_good_by_id(&good));
    }

    pub fn create(self : &mut Self, good: &Good) {
        let is_ok = Good::create(good, &mut self.connection);

        if is_ok {
            crate::views::display_new_good(good);
        } else {
            crate::views::display_err();
        }
    }

    pub fn update(self : &mut Self, good : &Good) {
        let is_ok = Good::update(good, &mut self.connection);

        if is_ok {
            crate::views::display_updated_good(good);
        } else {
            crate::views::display_err();
        }
    }

    pub fn delete(self : &mut Self, id: i32) {
        let is_ok = Good::delete_by_id(id, &mut self.connection);

        if is_ok {
            crate::views::display_deleted_good(id, is_ok);
        } else {
            crate::views::display_err();
        }
    }
}