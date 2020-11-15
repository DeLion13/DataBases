use crate::entities::Order;

// Viewing of changes in database with entity Order

pub fn display_deleted_order(order : &Order, is_deleted : bool) {
    println!(
        "Order {:#?} was{is_deleted} deleted",
        id = order,
        is_deleted = if !is_deleted { "n't" } else { "" }
    );
}

pub fn display_new_order(order : &Order) {
    println!("New order: {:#?}", order);
}

pub fn display_order_by_id(order : &Order) {
    println!("Order by id = {}: {:#?}", order.orders_id, order);
}

pub fn display_updated_order(order : &Order) {
    println!("Order was updated successfully: {:#?}", order);
}