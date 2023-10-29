use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use diesel::prelude::*;
use diesel::r2d2::*;
use diesel::sqlite::SqliteConnection;

use crate::entities::room::Room;
use crate::models::error::*;
use crate::schema::rooms::dsl::*;

#[derive(Debug)]
pub struct RoomService {
    db_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl RoomService {
    pub fn new(db_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        RoomService { db_pool }
    }

    pub fn create_room(&self, room: &Room) -> Result<Room> {
        log::info!("Creating room {}", room.id());
        let mut conn = self.db_pool.get()?;

        diesel::insert_into(rooms)
            .values(&room.clone())
            .execute(&mut conn)?;

        log::info!("Created room {}", room.id());
        Ok(room.clone())
    }

    pub fn get_room(&self, room_id: &str) -> Result<Room> {
        log::info!("Getting room {}", room_id);
        let conn = &mut self.db_pool.get()?;
        let result: QueryResult<Room> = rooms
            .filter(id.eq(room_id))
            .first(conn);

        log::info!("Got room {}", room_id);
        Ok(result.unwrap())
    }

    pub fn update_room(&self, room_id: &str, room_name: &str) -> Result<()> {
        log::info!("Updating room {}", room_id);
        let mut conn = self.db_pool.get()?;
        diesel::update(rooms.filter(id.eq(room_id)))
            .set(name.eq(room_name))
            .execute(&mut conn)?;

        log::info!("Updated room {}", room_id);
        Ok(())
    }

    pub fn delete_room(&self, room_id: &str) -> Result<()> {
        log::info!("Deleting room {}", room_id);
        let mut conn = self.db_pool.get()?;
        diesel::delete(rooms.filter(id.eq(room_id))).execute(&mut conn)?;

        log::info!("Deleted room {}", room_id);
        Ok(())
    }

    pub fn get_rooms(&self) -> Result<Vec<Room>> {
        log::info!("Getting all rooms");
        let mut conn = self.db_pool.get()?;
        let result = rooms.load::<Room>(&mut conn)?;

        log::info!("Got all rooms");
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use diesel::r2d2::{ConnectionManager, Pool};

    use crate::entities::room::Room;
    use crate::services::connection::establish_connection;

    use super::*;

    fn setup_database() -> Pool<ConnectionManager<SqliteConnection>> {
        let database_url = ":memory:"; // SQLite in-memory database
        establish_connection(Some(database_url.to_string()))
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
