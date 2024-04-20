
use diesel::prelude::*;
use crate::models::salary_history::{SalaryHistory, NewSalaryHistory, UpdateSalaryHistory};

pub fn create_salary_history(conn: &mut SqliteConnection, salary_history: NewSalaryHistory) -> Result<SalaryHistory, &'static str> {
    use crate::schema::salary_history;

    diesel::insert_into(salary_history::table)
        .values(&salary_history)
        .returning(SalaryHistory::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an salary_history")
}

pub fn get_salary_history(conn: &mut SqliteConnection, salary_history_id: i32) -> Result<Option<SalaryHistory>, &'static str> {
    use crate::schema::salary_history::dsl::*;

    salary_history.find(salary_history_id)
        .select(SalaryHistory::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an salary_history")
}

pub fn update_salary_history(conn: &mut SqliteConnection, salary_history_id: i32, _salary_history: &UpdateSalaryHistory) -> Result<SalaryHistory, &'static str> {
    use crate::schema::salary_history::dsl::*;

    diesel::update(salary_history.find(salary_history_id))
        .set(_salary_history)
        .get_result::<SalaryHistory>(conn)
        .map_err(|_| "Error updating an salary_history")
}

pub fn delete_salary_history(conn: &mut SqliteConnection, salary_history_id: i32) -> Result<bool, &'static str> {
    use crate::schema::salary_history::dsl::*;

    diesel::delete(salary_history.find(salary_history_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an salary_history")
}