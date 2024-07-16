use crate::schema::employee;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = employee)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Employee {
    pub id: i32,
    pub admission_date: Option<String>,
    pub payment_date: Option<String>,
    pub salary: Option<f64>,
    pub comission: Option<f64>,
    pub user_id: i32,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = employee)]
pub struct NewEmployee {
    pub admission_date: Option<String>,
    pub payment_date: Option<String>,
    pub salary: Option<f64>,
    pub comission: Option<f64>,
    pub user_id: i32,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = employee)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateEmployee {
    pub admission_date: Option<String>,
    pub payment_date: Option<String>,
    pub salary: Option<f64>,
    pub comission: Option<f64>,
    pub user_id: i32,
}
