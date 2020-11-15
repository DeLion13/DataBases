use crate::entities::Good;

// Viewing of changes in database with entity Good

pub fn display_deleted_good(good : &Good, is_deleted : bool) {
    println!(
        "Good {:#?} was{is_deleted} deleted",
        id = good,
        is_deleted = if !is_deleted { "n't" } else { "" }
    );
}

pub fn display_new_good(good : &Good) {
    println!("New good: {:#?}", good);
}

pub fn display_good_by_id(good : &Good) {
    println!("Good by id = {}: {:#?}", good.goods_id, good);
}

pub fn display_updated_good(good : &Good) {
    println!("Good was updated successfully: {:#?}", good);
}