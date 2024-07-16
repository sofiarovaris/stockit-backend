use crate::schema::product_nfe;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = product_nfe)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ProductNfe {
    pub id: i32,
    pub calculation_basis: Option<f64>,
    pub icms_intern: Option<f64>,
    pub ipi: Option<f64>,
    pub origin: Option<String>,
    pub cest: Option<f64>,
    pub ncm: Option<f64>,
    pub csosn: Option<f64>,
    pub product_id: i32,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = product_nfe)]
pub struct NewProductNfe {
    pub calculation_basis: Option<f64>,
    pub icms_intern: Option<f64>,
    pub ipi: Option<f64>,
    pub origin: Option<String>,
    pub cest: Option<f64>,
    pub ncm: Option<f64>,
    pub csosn: Option<f64>,
    pub product_id: i32,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = product_nfe)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateProductNfe {
    pub calculation_basis: Option<f64>,
    pub icms_intern: Option<f64>,
    pub ipi: Option<f64>,
    pub origin: Option<String>,
    pub cest: Option<f64>,
    pub ncm: Option<f64>,
    pub csosn: Option<f64>,
    pub product_id: i32,
}
