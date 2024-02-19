use diesel::pg::Pg;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::Pool;

use crate::error::Result;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub async fn establish_connection(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    log::info!("Establishing connection");

    // Use database_url if it is set, otherwise use DATABASE_URL
    log::info!("Using database url: {}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    log::info!("Creating database pool");
    let pool = Pool::new(manager).expect("Failed to create database pool");

    log::info!("Running migrations");
    run_migrations(&mut *pool.get().expect("Failed to get a connection from the pool")).expect("Failed to run migrations");

    log::info!("Connection established");
    pool
}

fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<()> {
    log::info!("Running migrations");

    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection
        .run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|e| panic!("Failed to run migrations: {}", e));

    log::info!("Migrations complete");
    Ok(())
}