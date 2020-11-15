use crate::models::entities::Category;
use crate::models::traits::*;
use postgres::Client;

pub struct CategoriesController {
    connection : Client
}

impl CategoriesController {
    pub fn new(connection : Client) -> Self {
        Self {connection}
    }

    pub fn get_by_id(self : &mut Self, id : i32) {
        let category = Category::find_by_id(id, &mut self.connection);

        crate::views::display_category_by_id(&category);
    }

    pub fn get_by_name(self : &mut Self, name : String) {
        let category = Category::find_by_name(name, &mut self.connection);

        category
            .iter()
            .for_each(|category| crate::views::display_category_by_id(&category));
    }

    pub fn create(self : &mut Self, category: &Category) {
        let is_ok = Category::create(category, &mut self.connection);

        if is_ok {
            crate::views::display_new_category(category);
        } else {
            crate::views::display_err();
        }
    }

    pub fn update(self : &mut Self, category: &Category) {
        let is_ok = Category::update(category, &mut self.connection);

        if is_ok {
            crate::views::display_updated_category(category);
        } else {
            crate::views::display_err();
        }
    }

    pub fn delete(self : &mut Self, id: i32) {
        let is_ok = Category::delete_by_id(id, &mut self.connection);

        if is_ok {
            crate::views::display_deleted_category(id, is_ok);
        } else {
            crate::views::display_err();
        }
    }
}