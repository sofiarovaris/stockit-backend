use crate::schema::client_user;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = client_user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ClientUser {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub state_registration: Option<String>,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub client_type: Option<String>,
    pub company_name: Option<String>,
    pub rg: Option<String>,
    pub address_id: Option<i32>,
    pub number: Option<i32>,
    pub complement: Option<String>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = client_user)]
pub struct NewClientUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub state_registration: Option<String>,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub client_type: Option<String>,
    pub company_name: Option<String>,
    pub rg: Option<String>,
    pub address_id: Option<i32>,
    pub number: Option<i32>,
    pub complement: Option<String>,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = client_user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateClientUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub state_registration: Option<String>,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub client_type: Option<String>,
    pub company_name: Option<String>,
    pub rg: Option<String>,
    pub address_id: Option<i32>,
    pub number: Option<i32>,
    pub complement: Option<String>,
}
