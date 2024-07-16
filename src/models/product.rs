use crate::schema::product;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = product)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Product {
    pub id: i32,
    pub name: Option<String>,
    pub code: Option<String>,
    pub observations: Option<String>,
    pub gross_weight: Option<f64>,
    pub net_weight: Option<f64>,
    pub batch_number: Option<String>,
    pub current_quantity: Option<i32>,
    pub provider_id: i32,
    pub producer_id: i32,
    pub unit_id: i32,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = product)]
pub struct NewProduct {
    pub name: Option<String>,
    pub code: Option<String>,
    pub observations: Option<String>,
    pub gross_weight: Option<f64>,
    pub net_weight: Option<f64>,
    pub batch_number: Option<String>,
    pub current_quantity: Option<i32>,
    pub provider_id: i32,
    pub producer_id: i32,
    pub unit_id: i32,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = product)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub code: Option<String>,
    pub observations: Option<String>,
    pub gross_weight: Option<f64>,
    pub net_weight: Option<f64>,
    pub batch_number: Option<String>,
    pub current_quantity: Option<i32>,
    pub provider_id: i32,
    pub producer_id: i32,
    pub unit_id: i32,
}
