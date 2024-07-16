use diesel::{expression::is_aggregate::No, prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;

use crate::{models::{role::NewRole, user::NewUser}, modules::{role::{create_role, exists_role}, user::{create_user, exists_admin}}};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migration(conn: &mut SqliteConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = format!("{}/{}", env::var("LOCALAPPDATA").unwrap(), env::var("DATABASE_URL").unwrap());
    
    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    run_migration(&mut connection);

    create_admin_user_if_not_exists(&mut connection);

    connection
}

pub fn create_admin_user_if_not_exists(conn: &mut SqliteConnection) {

    if let Ok(admin_role) = exists_role(conn) {
        if let None = admin_role {
            println!("ADMIN DOES NOT EXISTS");
            let _ = create_role(conn, NewRole {
                name: "admin".to_string(),
            });
            println!("ADMIN CREATED");
        }
    }

    if let Ok(admin_exists) = exists_admin(conn) {
        if let None = admin_exists {
            println!("ADMIN USER DOES NOT EXISTS");

            let new_user = NewUser {
                name: Some("Admin".to_string()),
                username: Some("admin".to_string()),
                password: "@superadmin123".to_string(),
                email: "admin@email.com".to_string(),
                rg: None,
                cpf: None,
                phone: None,
                number: None,
                complement: None,
                role_id: None,
                address_id: None,
            };

            let _ = create_user(conn, new_user);
            println!("ADMIN USER CREATED");
        }
    }

}