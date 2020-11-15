use crate::entities::Department;

// Viewing of changes in database with entity Department

pub fn display_deleted_department(id : i32, is_deleted : bool) {
    println!(
        "Department {:#?} was{is_deleted} deleted",
        id = id,
        is_deleted = if !is_deleted { "n't" } else { "" }
    );
}

pub fn display_new_department(department : &Department) {
    println!("New department: {:#?}", department);
}

pub fn display_department_by_id(department : &Department) {
    println!("Department by id = {}: {:#?}", department.departments_id, department);
}

pub fn display_updated_department(department : &Department) {
    println!("Department was updated successfully: {:#?}", department);
}