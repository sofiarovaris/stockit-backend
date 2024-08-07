use crate::schema::address;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
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
pub struct NewAddress {
    pub street: String,
    pub city: String,
    pub state: String,
    pub cep: String,
    pub neighborhood: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = address)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateAddress {
    pub street: String,
    pub city: String,
    pub state: String,
    pub cep: String,
    pub neighborhood: String,
}
