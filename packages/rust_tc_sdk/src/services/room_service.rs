use diesel::row::NamedRow;
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::*;
use crate::entities::room_entity::RoomEntity;
use crate::models::room::Room;
use crate::services::schema::rooms::dsl::{id as room_id, rooms};

pub struct RoomService {
  db_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl RoomService {
  pub fn new(db_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
    RoomService { db_pool }
  }

  pub fn create_room(&self, &room: Room) -> Result<Room, diesel::result::Error> {
    let new_room = RoomEntity { id: room.get_id(), name: room.get_name() };
    let mut conn = self.db_pool.get()?;

    let room: QueryResult<RoomEntity> = diesel::insert_into(rooms)
      .values(&new_room)
      .get_result(&mut conn)?;

    Ok(Room::new(room.room_id, room.name))
  }

  pub fn get_room(&self, id: &String) -> Result<RoomEntity, diesel::result::Error> {
    let mut conn = self.db_pool.get()?;
    let result = rooms.filter(room_id.eq(id)).first(&mut conn);

    match result {
      Ok(room) => Ok(room),
      Err(diesel::result::Error::NotFound) => Err("Room not found".into()),
      Err(err) => Err(err),
    }
  }

  pub fn update_room(&self, id: String, name: String) -> Result<(), diesel::result::Error> {
    let mut conn = self.db_pool.get()?;
    diesel::update(rooms.filter(room_id.eq(id)))
      .set(name.eq(&name))
      .execute(&mut conn)?;

    Ok(())
  }

  pub fn delete_room(&self, id: String) -> Result<(), diesel::result::Error> {
    let mut conn = self.db_pool.get()?;
    diesel::delete(rooms.filter(room_id.eq(id))).execute(&mut conn)?;

    Ok(())
  }

  pub fn get_rooms(&self) -> Result<Vec<RoomEntity>, diesel::result::Error> {
    let mut conn = self.db_pool.get()?;
    let result = rooms.load::<RoomEntity>(&mut conn)?;

    Ok(result)
  }
}
