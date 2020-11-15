use crate::entities::Category;

// Viewing of changes in database with entity Category

pub fn display_deleted_category(category : &Category, is_deleted : bool) {
    println!(
        "Category {:#?} was{is_deleted} deleted",
        id = category,
        is_deleted = if !is_deleted { "n't" } else { "" }
    );
}

pub fn display_new_category(category : &Category) {
    println!("New category: {:#?}", category);
}

pub fn display_category_by_id(category : &Category) {
    println!("Category by id = {}: {:#?}", category.categories_id, category);
}

pub fn display_updated_category(category : &Category) {
    println!("Category was updated successfully: {:#?}", category);
}