
use diesel::prelude::*;
use crate::models::product_nfe::{ProductNfe, NewProductNfe, UpdateProductNfe};

pub fn create_product_nfe(conn: &mut SqliteConnection, product_nfe: NewProductNfe) -> Result<ProductNfe, &'static str> {
    use crate::schema::product_nfe;

    diesel::insert_into(product_nfe::table)
        .values(&product_nfe)
        .returning(ProductNfe::as_returning())
        .get_result(conn)
        .map_err(|_| "Error creating an product_nfe")
}

pub fn get_product_nfe(conn: &mut SqliteConnection, product_nfe_id: i32) -> Result<Option<ProductNfe>, &'static str> {
    use crate::schema::product_nfe::dsl::*;

    product_nfe.find(product_nfe_id)
        .select(ProductNfe::as_select())
        .first(conn)
        .optional()
        .map_err(|_| "Error loading an product_nfe")
}

pub fn update_product_nfe(conn: &mut SqliteConnection, product_nfe_id: i32, _product_nfe: &UpdateProductNfe) -> Result<ProductNfe, &'static str> {
    use crate::schema::product_nfe::dsl::*;

    diesel::update(product_nfe.find(product_nfe_id))
        .set(_product_nfe)
        .get_result::<ProductNfe>(conn)
        .map_err(|_| "Error updating an product_nfe")
}

pub fn delete_product_nfe(conn: &mut SqliteConnection, product_nfe_id: i32) -> Result<bool, &'static str> {
    use crate::schema::product_nfe::dsl::*;

    diesel::delete(product_nfe.find(product_nfe_id))
        .execute(conn)
        .map(|rows_deleted| rows_deleted == 1)
        .map_err(|_| "Error deleting an product_nfe")
}