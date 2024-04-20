
use diesel::prelude::*;
use crate::models::payment::{Payment, NewPayment, UpdatePayment};

pub fn create_payment(conn: &mut SqliteConnection, payment: NewPayment) -> Result<Payment, &'static str> {
    use crate::schema::payment;

    diesel::insert_into(payment::table)
        .values(&payment)
        .returning(Payment::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an payment")
}

pub fn get_payment(conn: &mut SqliteConnection, payment_id: i32) -> Result<Option<Payment>, &'static str> {
    use crate::schema::payment::dsl::*;

    payment.find(payment_id)
        .select(Payment::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an payment")
}

pub fn update_payment(conn: &mut SqliteConnection, payment_id: i32, _payment: &UpdatePayment) -> Result<Payment, &'static str> {
    use crate::schema::payment::dsl::*;

    diesel::update(payment.find(payment_id))
        .set(_payment)
        .get_result::<Payment>(conn)
        .map_err(|_| "Error updating an payment")
}

pub fn delete_payment(conn: &mut SqliteConnection, payment_id: i32) -> Result<bool, &'static str> {
    use crate::schema::payment::dsl::*;

    diesel::delete(payment.find(payment_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an payment")
}