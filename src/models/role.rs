use crate::schema::role;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = role)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Role {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = role)]
pub struct NewRole {
    pub name: String,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = role)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateRole {
    pub name: String,
}
