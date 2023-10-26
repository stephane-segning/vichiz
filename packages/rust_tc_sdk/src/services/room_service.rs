use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use diesel::r2d2::*;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;

use crate::entities::room::Room;
use crate::models::error::*;

pub struct RoomService {
  db_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl RoomService {
  pub fn new(db_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
    RoomService { db_pool }
  }

  pub fn create_room(&self, room: &Room) -> Result<Room> {
    use crate::services::schema::rooms::dsl::*;

    let new_room = Room { id: room.id.clone(), name: room.name.clone() };
    let mut conn = self.db_pool.get()?;

    diesel::insert_into(rooms)
      .values(&new_room)
      .execute(&mut conn)?;

    Ok(room.clone())
  }

  pub fn get_room(&self, room_id: &str) -> Result<Room> {
    use crate::services::schema::rooms::dsl::*;

    let mut conn = self.db_pool.get()?;
    let result: QueryResult<Room> = rooms
      .filter(id.eq(room_id))
      .get_result(&mut conn)?;

    Ok(result.unwrap())
  }

  pub fn update_room(&self, room_id: &str, room_name: &str) -> Result<()> {
    use crate::services::schema::rooms::dsl::*;

    let mut conn = self.db_pool.get()?;
    diesel::update(rooms.filter(id.eq(room_id)))
      .set(name.eq(room_name))
      .execute(&mut conn)?;

    Ok(())
  }

  pub fn delete_room(&self, room_id: &str) -> Result<()> {
    use crate::services::schema::rooms::dsl::*;

    let mut conn = self.db_pool.get()?;
    diesel::delete(rooms.filter(id.eq(room_id))).execute(&mut conn)?;

    Ok(())
  }

  pub fn get_rooms(&self) -> Result<Vec<Room>> {
    use crate::services::schema::rooms::dsl::*;

    let mut conn = self.db_pool.get()?;
    let result = rooms.load::<Room>(&mut conn)?;

    Ok(result)
  }
}
