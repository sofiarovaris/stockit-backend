use crate::schema::provider;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = provider)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Provider {
    pub id: i32,
    pub name: Option<String>,
    pub company_name: Option<String>,
    pub state_registration: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub cnpj: Option<String>,
    pub bank_reference: Option<String>,
    pub address_id: Option<i32>,
    pub number: Option<i32>,
    pub complement: Option<String>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = provider)]
pub struct NewProvider {
    pub name: Option<String>,
    pub company_name: Option<String>,
    pub state_registration: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub cnpj: Option<String>,
    pub bank_reference: Option<String>,
    pub address_id: Option<i32>,
    pub number: Option<i32>,
    pub complement: Option<String>,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = provider)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateProvider {
    pub name: Option<String>,
    pub company_name: Option<String>,
    pub state_registration: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub cnpj: Option<String>,
    pub bank_reference: Option<String>,
    pub address_id: Option<i32>,
    pub number: Option<i32>,
    pub complement: Option<String>,
}
