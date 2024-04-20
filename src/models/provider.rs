use crate::schema::provider;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = provider)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Provider {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub cnpj: String,
    pub company_name: String,
    pub state_registration: String,
    pub address_id: i32,
    pub bank_reference: String,
}

#[derive(Insertable)]
#[diesel(table_name = provider)]
pub struct NewProvider<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub cnpj: &'a str,
    pub company_name: &'a str,
    pub state_registration: &'a str,
    pub address_id: &'a i32,
    pub bank_reference: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = provider)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateProvider<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub cnpj: &'a str,
    pub company_name: &'a str,
    pub state_registration: &'a str,
    pub address_id: &'a i32,
    pub bank_reference: &'a str,
}
