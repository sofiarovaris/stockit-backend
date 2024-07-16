
use diesel::prelude::*;
use crate::models::client_user::{ClientUser, NewClientUser, UpdateClientUser};

pub fn create_client_user(conn: &mut SqliteConnection, client_user: NewClientUser) -> Result<ClientUser, &'static str> {
    use crate::schema::client_user;

    diesel::insert_into(client_user::table)
        .values(&client_user)
        .returning(ClientUser::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an client_user")
}

pub fn get_client_user(conn: &mut SqliteConnection, client_user_id: i32) -> Result<Option<ClientUser>, &'static str> {
    use crate::schema::client_user::dsl::*;

    client_user.find(client_user_id)
        .select(ClientUser::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an client_user")
}

pub fn update_client_user(conn: &mut SqliteConnection, client_user_id: i32, _client_user: &UpdateClientUser) -> Result<ClientUser, &'static str> {
    use crate::schema::client_user::dsl::*;

    diesel::update(client_user.find(client_user_id))
        .set(_client_user)
        .get_result::<ClientUser>(conn)
        .map_err(|_| "Error updating an client_user")
}

pub fn delete_client_user(conn: &mut SqliteConnection, client_user_id: i32) -> Result<bool, &'static str> {
    use crate::schema::client_user::dsl::*;

    diesel::delete(client_user.find(client_user_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an client_user")
}

pub fn get_client_users(conn: &mut SqliteConnection) -> Result<Vec<ClientUser>, &'static str> {
    use crate::schema::client_user::dsl::*;

    client_user.select(ClientUser::as_select())
        .load(conn)
        .map_err(|_| "Error loading client_users")
}

pub fn remove_client_user_address(conn: &mut SqliteConnection, client_user_id_param: i32) -> Result<ClientUser, &'static str> {
    use crate::schema::client_user::dsl::*;

    diesel::update(client_user.find(client_user_id_param))
        .set(address_id.eq(None::<i32>))
        .get_result::<ClientUser>(conn)
        .map_err(|_| "Error removing user address")
}