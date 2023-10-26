use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use diesel::prelude::*;
use diesel::r2d2::*;
use diesel::sqlite::SqliteConnection;

use crate::entities::room::Room;
use crate::models::error::*;
use crate::schema::rooms::dsl::*;

pub struct RoomService {
    db_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl RoomService {
    pub fn new(db_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        RoomService { db_pool }
    }

    pub fn create_room(&self, room: &Room) -> Result<Room> {
        let mut conn = self.db_pool.get()?;

        diesel::insert_into(rooms)
            .values(&room.clone())
            .execute(&mut conn)?;

        Ok(room.clone())
    }

    pub fn get_room(&self, room_id: &str) -> Result<Room> {
        let conn = &mut self.db_pool.get()?;
        let result: QueryResult<Room> = rooms
            .filter(id.eq(room_id))
            .first(conn);

        Ok(result.unwrap())
    }

    pub fn update_room(&self, room_id: &str, room_name: &str) -> Result<()> {
        let mut conn = self.db_pool.get()?;
        diesel::update(rooms.filter(id.eq(room_id)))
            .set(name.eq(room_name))
            .execute(&mut conn)?;

        Ok(())
    }

    pub fn delete_room(&self, room_id: &str) -> Result<()> {
        let mut conn = self.db_pool.get()?;
        diesel::delete(rooms.filter(id.eq(room_id))).execute(&mut conn)?;

        Ok(())
    }

    pub fn get_rooms(&self) -> Result<Vec<Room>> {
        let mut conn = self.db_pool.get()?;
        let result = rooms.load::<Room>(&mut conn)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use diesel::r2d2::{ConnectionManager, Pool};

    use crate::entities::room::Room;

    use super::*;

    fn setup_database() -> Pool<ConnectionManager<SqliteConnection>> {
        let manager = ConnectionManager::<SqliteConnection>::new("sqlite::memory:");
        let pool = Pool::new(manager).expect("Failed to create pool.");
        let connection = pool.get().expect("Failed to get connection from pool.");

        // Create the `rooms` table in the in-memory database.
        // This assumes you have a migration script or similar to set up the table.
        // You might need to adjust this if your setup is different.
        // diesel::migrations::run_pending_migrations(&connection).unwrap();

        pool
    }

    #[test]
    fn test_create_room() {
        let pool = setup_database();
        let service = RoomService::new(pool);
        let room = Room::from(("123".to_string(), "Test Room".to_string()));

        let result = service.create_room(&room);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_room() {
        let pool = setup_database();
        let service = RoomService::new(pool);
        let room = Room::from(("123".to_string(), "Test Room".to_string()));

        service.create_room(&room).unwrap();
        let fetched_room = service.get_room("123");
        assert_eq!(fetched_room.unwrap().name(), "Test Room");
    }

    // Similarly, you can add tests for update_room, delete_room, and get_rooms.

    // Note: The tests are very basic and do not cover all edge cases.
    // It's recommended to add more tests and assertions to cover various scenarios.
}
