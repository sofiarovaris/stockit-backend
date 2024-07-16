
use diesel::prelude::*;
use crate::models::employee::{Employee, NewEmployee, UpdateEmployee};

pub fn create_employee(conn: &mut SqliteConnection, employee: NewEmployee) -> Result<Employee, &'static str> {
    use crate::schema::employee;

    diesel::insert_into(employee::table)
        .values(&employee)
        .returning(Employee::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an employee")
}

pub fn get_employee(conn: &mut SqliteConnection, employee_id: i32) -> Result<Option<Employee>, &'static str> {
    use crate::schema::employee::dsl::*;

    employee.find(employee_id)
        .select(Employee::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an employee")
}

pub fn update_employee(conn: &mut SqliteConnection, employee_id: i32, _employee: &UpdateEmployee) -> Result<Employee, &'static str> {
    use crate::schema::employee::dsl::*;

    diesel::update(employee.find(employee_id))
        .set(_employee)
        .get_result::<Employee>(conn)
        .map_err(|_| "Error updating an employee")
}

pub fn delete_employee(conn: &mut SqliteConnection, employee_id: i32) -> Result<bool, &'static str> {
    use crate::schema::employee::dsl::*;

    diesel::delete(employee.find(employee_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an employee")
}

pub fn get_employee_by_user_id(conn: &mut SqliteConnection, user_id_param: i32) -> Result<Option<Employee>, &'static str> {
    use crate::schema::employee::dsl::*;

    employee.filter(user_id.eq(user_id_param))
        .select(Employee::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading employee by user ID")
}