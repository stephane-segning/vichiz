use diesel::r2d2::*;
use diesel::sqlite::SqliteConnection;

pub fn establish_connection(database_url: String) -> Pool<ConnectionManager<SqliteConnection>> {
  // Use database_url if it is set, otherwise use DATABASE_URL
  let manager = ConnectionManager::<SqliteConnection>::new(database_url);

  Pool::new(manager).expect("Failed to create database pool")
}
