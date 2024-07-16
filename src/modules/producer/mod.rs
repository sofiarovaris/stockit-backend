
use diesel::prelude::*;
use crate::models::producer::{Producer, NewProducer, UpdateProducer};

pub fn create_producer(conn: &mut SqliteConnection, producer: NewProducer) -> Result<Producer, &'static str> {
    use crate::schema::producer;

    diesel::insert_into(producer::table)
        .values(&producer)
        .returning(Producer::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an producer")
}

pub fn get_producer(conn: &mut SqliteConnection, producer_id: i32) -> Result<Option<Producer>, &'static str> {
    use crate::schema::producer::dsl::*;

    producer.find(producer_id)
        .select(Producer::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an producer")
}

pub fn get_producers(conn: &mut SqliteConnection) -> Result<Vec<Producer>, &'static str> {
    use crate::schema::producer::dsl::*;

    producer.select(Producer::as_select())
        .load(conn)
        .map_err(|_| "Error loading producers")
}

pub fn update_producer(conn: &mut SqliteConnection, producer_id: i32, _producer: &UpdateProducer) -> Result<Producer, &'static str> {
    use crate::schema::producer::dsl::*;

    diesel::update(producer.find(producer_id))
        .set(_producer)
        .get_result::<Producer>(conn)
        .map_err(|_| "Error updating an producer")
}

pub fn delete_producer(conn: &mut SqliteConnection, producer_id: i32) -> Result<bool, &'static str> {
    use crate::schema::producer::dsl::*;

    diesel::delete(producer.find(producer_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an producer")
}