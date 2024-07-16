use crate::schema::unit;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = unit)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Unit {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = unit)]
pub struct NewUnit {
    pub name: String,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = unit)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateUnit {
    pub name: String,
}
