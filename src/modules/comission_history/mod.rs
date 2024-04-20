
use diesel::prelude::*;
use crate::models::comission_history::{ComissionHistory, NewComissionHistory, UpdateComissionHistory};

pub fn create_comission_history(conn: &mut SqliteConnection, comission_history: NewComissionHistory) -> Result<ComissionHistory, &'static str> {
    use crate::schema::comission_history;

    diesel::insert_into(comission_history::table)
        .values(&comission_history)
        .returning(ComissionHistory::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an comission_history")
}

pub fn get_comission_history(conn: &mut SqliteConnection, comission_history_id: i32) -> Result<Option<ComissionHistory>, &'static str> {
    use crate::schema::comission_history::dsl::*;

    comission_history.find(comission_history_id)
        .select(ComissionHistory::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an comission_history")
}

pub fn update_comission_history(conn: &mut SqliteConnection, comission_history_id: i32, _comission_history: &UpdateComissionHistory) -> Result<ComissionHistory, &'static str> {
    use crate::schema::comission_history::dsl::*;

    diesel::update(comission_history.find(comission_history_id))
        .set(_comission_history)
        .get_result::<ComissionHistory>(conn)
        .map_err(|_| "Error updating an comission_history")
}

pub fn delete_comission_history(conn: &mut SqliteConnection, comission_history_id: i32) -> Result<bool, &'static str> {
    use crate::schema::comission_history::dsl::*;

    diesel::delete(comission_history.find(comission_history_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an comission_history")
}