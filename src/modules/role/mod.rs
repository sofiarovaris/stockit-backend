
use diesel::prelude::*;
use crate::models::role::{Role, NewRole, UpdateRole};

pub fn create_role(conn: &mut SqliteConnection, role: NewRole) -> Result<Role, &'static str> {
    use crate::schema::role;

    diesel::insert_into(role::table)
        .values(&role)
        .returning(Role::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an role")
}

pub fn get_role(conn: &mut SqliteConnection, role_id: i32) -> Result<Option<Role>, &'static str> {
    use crate::schema::role::dsl::*;

    role.find(role_id)
        .select(Role::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an role")
}

pub fn get_roles(conn: &mut SqliteConnection) -> Result<Vec<Role>, &'static str> {
    use crate::schema::role::dsl::*;

    role.select(Role::as_select())
        .load(conn)
        .map_err(|_| "Error loading roles")
}

pub fn update_role(conn: &mut SqliteConnection, role_id: i32, _role: &UpdateRole) -> Result<Role, &'static str> {
    use crate::schema::role::dsl::*;

    diesel::update(role.find(role_id))
        .set(_role)
        .get_result::<Role>(conn)
        .map_err(|_| "Error updating an role")
}

pub fn delete_role(conn: &mut SqliteConnection, role_id: i32) -> Result<bool, &'static str> {
    use crate::schema::role::dsl::*;

    diesel::delete(role.find(role_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an role")
}

pub fn exists_role(conn: &mut SqliteConnection) -> Result<Option<Role>, &'static str> {
    use crate::schema::role::dsl::*;

    role.filter(name.eq("admin"))
        .first::<Role>(conn)
        .optional()
        .map_err(|_| "Error finding admin role")
}