
use diesel::prelude::*;
use crate::models::product::{Product, NewProduct, UpdateProduct};

pub fn create_product(conn: &mut SqliteConnection, product: NewProduct) -> Result<Product, &'static str> {
    use crate::schema::product;

    diesel::insert_into(product::table)
        .values(&product)
        .returning(Product::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an product")
}

pub fn get_product(conn: &mut SqliteConnection, product_id: i32) -> Result<Option<Product>, &'static str> {
    use crate::schema::product::dsl::*;

    product.find(product_id)
        .select(Product::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an product")
}

pub fn update_product(conn: &mut SqliteConnection, product_id: i32, _product: &UpdateProduct) -> Result<Product, &'static str> {
    use crate::schema::product::dsl::*;

    diesel::update(product.find(product_id))
        .set(_product)
        .get_result::<Product>(conn)
        .map_err(|_| "Error updating an product")
}

pub fn delete_product(conn: &mut SqliteConnection, product_id: i32) -> Result<bool, &'static str> {
    use crate::schema::product::dsl::*;

    diesel::delete(product.find(product_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an product")
}

pub fn get_products(conn: &mut SqliteConnection) -> Result<Vec<Product>, &'static str> {
    use crate::schema::product::dsl::*;

    product.select(Product::as_select())
        .load(conn)
        .map_err(|_| "Error loading products")
}