use crate::schema::salary_history;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = salary_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SalaryHistory {
    pub id: i32,
    pub salary: f64,
    pub date : String,
    pub employee_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = salary_history)]
pub struct NewSalaryHistory<'a> {
    pub salary: &'a f64,
    pub date: &'a str,
    pub employee_id: &'a i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = salary_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateSalaryHistory<'a> {
    pub salary: &'a f64,
    pub date: &'a str,
    pub employee_id: &'a i32,
}
