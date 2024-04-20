
use diesel::prelude::*;
use crate::models::client::{Client, NewClient, UpdateClient};

pub fn create_client(conn: &mut SqliteConnection, client: NewClient) -> Result<Client, &'static str> {
    use crate::schema::client;

    diesel::insert_into(client::table)
        .values(&client)
        .returning(Client::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an client")
}

pub fn get_client(conn: &mut SqliteConnection, client_id: i32) -> Result<Option<Client>, &'static str> {
    use crate::schema::client::dsl::*;

    client.find(client_id)
        .select(Client::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an client")
}

pub fn update_client(conn: &mut SqliteConnection, client_id: i32, _client: &UpdateClient) -> Result<Client, &'static str> {
    use crate::schema::client::dsl::*;

    diesel::update(client.find(client_id))
        .set(_client)
        .get_result::<Client>(conn)
        .map_err(|_| "Error updating an client")
}

pub fn delete_client(conn: &mut SqliteConnection, client_id: i32) -> Result<bool, &'static str> {
    use crate::schema::client::dsl::*;

    diesel::delete(client.find(client_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an client")
}