use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use diesel::prelude::*;
use diesel::r2d2::*;
use diesel::sqlite::SqliteConnection;

use crate::entities::room::Room;
use crate::models::error::*;
use crate::services::schema::rooms::dsl::*;

pub struct RoomService {
    db_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl RoomService {
    pub fn new(db_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        RoomService { db_pool }
    }

    pub fn create_room(&self, room: &Room) -> Result<Room> {
        let new_room = Room::new(room.get_id(), room.get_name());
        let mut conn = self.db_pool.get()?;

        diesel::insert_into(rooms)
            .values(&new_room)
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
