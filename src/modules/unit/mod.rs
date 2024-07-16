
use diesel::prelude::*;
use crate::models::unit::{Unit, NewUnit, UpdateUnit};

pub fn create_unit(conn: &mut SqliteConnection, unit: NewUnit) -> Result<Unit, &'static str> {
    use crate::schema::unit;

    diesel::insert_into(unit::table)
        .values(&unit)
        .returning(Unit::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an unit")
}

pub fn get_unit(conn: &mut SqliteConnection, unit_id: i32) -> Result<Option<Unit>, &'static str> {
    use crate::schema::unit::dsl::*;

    unit.find(unit_id)
        .select(Unit::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an unit")
}

pub fn get_units(conn: &mut SqliteConnection) -> Result<Vec<Unit>, &'static str> {
    use crate::schema::unit::dsl::*;

    unit.select(Unit::as_select())
        .load(conn)
        .map_err(|_| "Error loading units")
}

pub fn update_unit(conn: &mut SqliteConnection, unit_id: i32, _unit: &UpdateUnit) -> Result<Unit, &'static str> {
    use crate::schema::unit::dsl::*;

    diesel::update(unit.find(unit_id))
        .set(_unit)
        .get_result::<Unit>(conn)
        .map_err(|_| "Error updating an unit")
}

pub fn delete_unit(conn: &mut SqliteConnection, unit_id: i32) -> Result<bool, &'static str> {
    use crate::schema::unit::dsl::*;

    diesel::delete(unit.find(unit_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an unit")
}