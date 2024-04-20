use crate::schema::client;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = client)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub cell: String,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub type_: String,
    pub company_name: Option<String>,
    pub rg: Option<String>,
    pub address_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = client)]
pub struct NewClient<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub cell: &'a str,
    pub cpf: Option<&'a str>,
    pub cnpj: Option<&'a str>,
    pub type_: &'a str,
    pub company_name: Option<&'a str>,
    pub rg: Option<&'a str>,
    pub address_id: &'a i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = client)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateClient<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub cell: &'a str,
    pub cpf: Option<&'a str>,
    pub cnpj: Option<&'a str>,
    pub type_: &'a str,
    pub company_name: Option<&'a str>,
    pub rg: Option<&'a str>,
    pub address_id: &'a i32,
}
