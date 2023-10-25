use std::option::Option;

use diesel::r2d2::*;
use diesel::sqlite::SqliteConnection;

pub fn establish_connection(database_url: Option<String>) -> Pool<ConnectionManager<SqliteConnection>> {
  // Use database_url if it is set, otherwise use DATABASE_URL
  let url = match database_url {
    Some(str) => str,
    None => panic!("DATABASE_URL must be set"),
  };

  let manager = ConnectionManager::<SqliteConnection>::new(url);

  Pool::new(manager).expect("Failed to create database pool")
}
