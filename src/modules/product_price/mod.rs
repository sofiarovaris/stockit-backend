
use diesel::prelude::*;
use crate::models::product_price::{ProductPrice, NewProductPrice, UpdateProductPrice};

pub fn create_product_price(conn: &mut SqliteConnection, product_price: NewProductPrice) -> Result<ProductPrice, &'static str> {
    use crate::schema::product_price;

    diesel::insert_into(product_price::table)
        .values(&product_price)
        .returning(ProductPrice::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an product_price")
}

pub fn get_product_price(conn: &mut SqliteConnection, product_price_id: i32) -> Result<Option<ProductPrice>, &'static str> {
    use crate::schema::product_price::dsl::*;

    product_price.find(product_price_id)
        .select(ProductPrice::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an product_price")
}

pub fn update_product_price(conn: &mut SqliteConnection, product_price_id: i32, _product_price: &UpdateProductPrice) -> Result<ProductPrice, &'static str> {
    use crate::schema::product_price::dsl::*;

    diesel::update(product_price.find(product_price_id))
        .set(_product_price)
        .get_result::<ProductPrice>(conn)
        .map_err(|_| "Error updating an product_price")
}

pub fn delete_product_price(conn: &mut SqliteConnection, product_price_id: i32) -> Result<bool, &'static str> {
    use crate::schema::product_price::dsl::*;

    diesel::delete(product_price.find(product_price_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an product_price")
}