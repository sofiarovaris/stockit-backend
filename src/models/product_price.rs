use crate::schema::product_price;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = product_price)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ProductPrice {
    pub id: i32,
    pub price_cost: Option<f64>,
    pub sale_price: Option<f64>,
    pub profit_margin: Option<f64>,
    pub product_id: i32,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = product_price)]
pub struct NewProductPrice {
    pub price_cost: Option<f64>,
    pub sale_price: Option<f64>,
    pub profit_margin: Option<f64>,
    pub product_id: i32,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = product_price)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateProductPrice {
    pub price_cost: Option<f64>,
    pub sale_price: Option<f64>,
    pub profit_margin: Option<f64>,
    pub product_id: i32,
}
