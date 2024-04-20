use crate::schema::employee;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = employee)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Employee {
    pub id: i32,
    pub admission_date: String,
    pub payment_date: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = employee)]
pub struct NewEmployee<'a> {
    pub admission_date: &'a str,
    pub payment_date: &'a str,
    pub user_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = employee)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateEmployee<'a> {
    pub admission_date: &'a str,
    pub payment_date: &'a str,
    pub user_id: i32,
}
