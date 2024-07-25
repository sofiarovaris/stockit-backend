use crate::schema::user;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: String,
    pub email: String,
    pub rg: Option<String>,
    pub cpf: Option<String>,
    pub phone: Option<String>,
    pub number: Option<i32>,
    pub complement: Option<String>,
    pub role_id: Option<i32>,
    pub address_id: Option<i32>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: String,
    pub rg: Option<String>,
    pub cpf: Option<String>,
    pub phone: Option<String>,
    pub number: Option<i32>,
    pub complement: Option<String>,
    pub role_id: Option<i32>,
    pub address_id: Option<i32>,
    pub email: String,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: String,
    pub rg: Option<String>,
    pub cpf: Option<String>,
    pub phone: Option<String>,
    pub number: Option<i32>,
    pub complement: Option<String>,
    pub role_id: Option<i32>,
    pub address_id: Option<i32>,
}
