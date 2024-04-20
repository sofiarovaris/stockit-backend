use crate::schema::user;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String, 
    pub email: String,
    pub rg: String,
    pub cpf: String,
    pub phone: String,
    pub username: String,
    pub password: String,
    pub number: i32,
    pub complement: Option<String>,
    pub address_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub rg: &'a str,
    pub cpf: &'a str,
    pub phone: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub number: &'a i32,
    pub complement: Option<&'a str>,
    pub address_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub rg: &'a str,
    pub cpf: &'a str,
    pub phone: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub number: &'a i32,
    pub complement: Option<&'a str>,
    pub address_id: i32,
}
