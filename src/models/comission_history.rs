use crate::schema::comission_history;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = comission_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ComissionHistory {
    pub id: i32,
    pub comission: f64,
    pub date : String,
    pub employee_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = comission_history)]
pub struct NewComissionHistory<'a> {
    pub comission: &'a f64,
    pub date: &'a str,
    pub employee_id: &'a i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = comission_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateComissionHistory<'a> {
    pub comission: &'a f64,
    pub date: &'a str,
    pub employee_id: &'a i32,
}
