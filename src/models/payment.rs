use crate::schema::payment;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = payment)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Payment {
    pub id: i32,
    pub amount: f64,
    pub month: i32,
    pub year: i32,
    pub employee_id: i32,

}

#[derive(Insertable)]
#[diesel(table_name = payment)]
pub struct NewPayment<'a> {
    pub amount: &'a f64,
    pub month: &'a i32,
    pub year: &'a i32,
    pub employee_id: &'a i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = payment)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdatePayment<'a> {
    pub amount: &'a f64,
    pub month: &'a i32,
    pub year: &'a i32,
    pub employee_id: &'a i32,
}
