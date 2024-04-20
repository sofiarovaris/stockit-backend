
use diesel::prelude::*;
use crate::models::address::{Address, NewAddress, UpdateAddress};

pub fn create_address(conn: &mut SqliteConnection, address: NewAddress) -> Result<Address, &'static str> {
    use crate::schema::address;

    diesel::insert_into(address::table)
        .values(&address)
        .returning(Address::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an address")
}

pub fn get_address(conn: &mut SqliteConnection, address_id: i32) -> Result<Option<Address>, &'static str> {
    use crate::schema::address::dsl::*;

    address.find(address_id)
        .select(Address::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an address")
}

pub fn get_address_by_cep(conn: &mut SqliteConnection, _cep: &str) -> Result<Address, &'static str> {
    use crate::schema::address::dsl::*;

    address.filter(cep.eq(_cep))
        .select(Address::as_select())
        .first(conn)
        .map_err(|_| "Error loading an address by CEP")
}

pub fn update_address(conn: &mut SqliteConnection, address_id: i32, _address: &UpdateAddress) -> Result<Address, &'static str> {
    use crate::schema::address::dsl::*;

    diesel::update(address.find(address_id))
        .set(_address)
        .get_result::<Address>(conn)
        .map_err(|_| "Error updating an address")
}

pub fn delete_address(conn: &mut SqliteConnection, address_id: i32) -> Result<bool, &'static str> {
    use crate::schema::address::dsl::*;

    diesel::delete(address.find(address_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an address")
}