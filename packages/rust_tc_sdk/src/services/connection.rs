use diesel::r2d2::*;
use diesel::sqlite::{Sqlite, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::models::error::Result;
use crate::services::database_url::get_database_path;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

// This is a type alias for the r2d2 connection pool type we'll be using
#[inline]
pub fn establish_connection(user_url: Option<String>) -> Pool<ConnectionManager<SqliteConnection>> {
    log::info!("Establishing connection");

    let database_url = match user_url {
        Some(url) if url.len() > 0 => url,
        _ => match get_database_path() {
            Some(path) => path,
            None => panic!("Failed to get database path")
        }
    };

    // Use database_url if it is set, otherwise use DATABASE_URL
    log::info!("Using database url: {}", database_url);
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    log::info!("Creating database pool");
    let pool = Pool::new(manager).expect("Failed to create database pool");

    log::info!("Running migrations");
    run_migrations(&mut *pool.get().expect("Failed to get a connection from the pool")).expect("Failed to run migrations");

    log::info!("Connection established");
    pool
}

fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<()> {
    log::info!("Running migrations");

    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection
        .run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|e| panic!("Failed to run migrations {}", e));

    log::info!("Migrations complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_establish_connection_success() {
        // Create a temporary SQLite database for testing
        let db_path = "./test.db";
        let _ = fs::remove_file(db_path); // Clean up any existing test database

        let pool = establish_connection(Some(db_path.to_string()));

        // Assert the pool was created successfully by attempting a connection
        pool.get().expect("Failed to get a connection from the pool");

        let _ = fs::remove_file(db_path); // Clean up after the test
    }

    // Optionally, you could add more tests for edge cases, invalid URLs, etc.
    // But handling them might require changing the function to return a Result.
}
