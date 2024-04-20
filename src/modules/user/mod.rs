
use diesel::prelude::*;
use crate::models::user::{User, NewUser, UpdateUser};

pub fn create_user(conn: &mut SqliteConnection, user: NewUser) -> Result<User, &'static str> {
    use crate::schema::user;

    diesel::insert_into(user::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an user")
}

pub fn get_user(conn: &mut SqliteConnection, user_id: i32) -> Result<Option<User>, &'static str> {
    use crate::schema::user::dsl::*;

    user.find(user_id)
        .select(User::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an user")
}

pub fn update_user(conn: &mut SqliteConnection, user_id: i32, _user: &UpdateUser) -> Result<User, &'static str> {
    use crate::schema::user::dsl::*;

    diesel::update(user.find(user_id))
        .set(_user)
        .get_result::<User>(conn)
        .map_err(|_| "Error updating an user")
}

pub fn delete_user(conn: &mut SqliteConnection, user_id: i32) -> Result<bool, &'static str> {
    use crate::schema::user::dsl::*;

    diesel::delete(user.find(user_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an user")
}