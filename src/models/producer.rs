use crate::schema::producer;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = producer)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Producer {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = producer)]
pub struct NewProducer {
    pub name: String,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = producer)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateProducer {
    pub name: String,
}
