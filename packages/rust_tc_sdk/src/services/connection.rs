use diesel::r2d2::*;
use diesel::sqlite::SqliteConnection;

#[inline]
pub fn establish_connection(database_url: String) -> Pool<ConnectionManager<SqliteConnection>> {
  // Use database_url if it is set, otherwise use DATABASE_URL
  let manager = ConnectionManager::<SqliteConnection>::new(database_url);

  Pool::new(manager).expect("Failed to create database pool")
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  #[test]
  fn test_establish_connection_success() {
    // Create a temporary SQLite database for testing
    let db_path = "./test.db";
    let _ = fs::remove_file(db_path); // Clean up any existing test database

    let pool = establish_connection(db_path.to_string());

    // Assert the pool was created successfully by attempting a connection
    pool.get().expect("Failed to get a connection from the pool");

    let _ = fs::remove_file(db_path); // Clean up after the test
  }

  // Optionally, you could add more tests for edge cases, invalid URLs, etc.
  // But handling them might require changing the function to return a Result.
}
