use crate::schema::address;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = address)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Address {
    pub id: i32,
    pub street: String,
    pub city: String,
    pub state: String,
    pub cep: String,
    pub neighborhood: String,
}

#[derive(Insertable)]
#[diesel(table_name = address)]
pub struct NewAddress<'a> {
    pub street: &'a str,
    pub city: &'a str,
    pub state: &'a str,
    pub cep: &'a str,
    pub neighborhood: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = address)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateAddress<'a> {
    pub id: i32,
    pub street: &'a str,
    pub city: &'a str,
    pub state: &'a str,
    pub cep: &'a str,
    pub neighborhood: &'a str,
}
