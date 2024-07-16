
use diesel::prelude::*;
use crate::models::provider::{Provider, NewProvider, UpdateProvider};

pub fn create_provider(conn: &mut SqliteConnection, provider: NewProvider) -> Result<Provider, &'static str> {
    use crate::schema::provider;

    diesel::insert_into(provider::table)
        .values(&provider)
        .returning(Provider::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an provider")
}

pub fn get_provider(conn: &mut SqliteConnection, provider_id: i32) -> Result<Option<Provider>, &'static str> {
    use crate::schema::provider::dsl::*;

    provider.find(provider_id)
        .select(Provider::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an provider")
}

pub fn get_providers(conn: &mut SqliteConnection) -> Result<Vec<Provider>, &'static str> {
    use crate::schema::provider::dsl::*;

    provider.select(Provider::as_select())
        .load(conn)
        .map_err(|_| "Error loading providers")
}

pub fn update_provider(conn: &mut SqliteConnection, provider_id: i32, _provider: &UpdateProvider) -> Result<Provider, &'static str> {
    use crate::schema::provider::dsl::*;

    diesel::update(provider.find(provider_id))
        .set(_provider)
        .get_result::<Provider>(conn)
        .map_err(|_| "Error updating an provider")
}


pub fn delete_provider(conn: &mut SqliteConnection, provider_id: i32) -> Result<bool, &'static str> {
    use crate::schema::provider::dsl::*;

    diesel::delete(provider.find(provider_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an provider")
}

pub fn remove_provider_address(conn: &mut SqliteConnection, provider_id: i32) -> Result<Provider, &'static str> {
    use crate::schema::provider::dsl::*;

    diesel::update(provider.find(provider_id))
        .set(address_id.eq(None::<i32>))
        .get_result::<Provider>(conn)
        .map_err(|_| "Error removing provider address")
}