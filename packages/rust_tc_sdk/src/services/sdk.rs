use neon::prelude::*;
use uuid::Uuid;

use crate::models::error::Error;
use crate::models::room::{Room, RoomOption};
use crate::services::connection::establish_connection;
use crate::services::noise_key_service::NoiseKeyService;
use crate::services::room_service::RoomService;

pub struct RustSDK {
  room_service: RoomService,
  noise_key_service: NoiseKeyService,
}

pub struct RustSDKOptions {
  pub db_url: Option<String>,
}

impl RustSDK {
  pub fn new(options: RustSDKOptions) -> Self {
    let db_pool = establish_connection(options.db_url);

    // Initialize the NoiseKeyService with the connection pool.
    let noise_key_service = NoiseKeyService::new(db_pool.clone());
    let room_service = RoomService::new(db_pool.clone());

    Self {
      room_service,
      noise_key_service,
    }
  }

  pub fn create_room(&mut self, options: RoomOption) -> Result<Room, Error> {
    // Create noise keys for the room.
    let room_id = match options.id {
      None => Uuid::new_v4().to_string(),
      Some(x) => x
    };
    self.noise_key_service.create_key(&room_id)?;

    // Create a RoomEntity and persist it.
    let room = Room::new(room_id.clone(), options.name.clone());
    self.room_service.create_room(room.clone())?;

    // Create a Room object and store it in the SDK's internal state.
    self.rooms.insert(room_id.clone(), room.clone());

    Ok(room)
  }

  pub fn start_room(&self, room_id: &String) -> Result<(), Error> {
    // Start the room.
    let room = match self.room_service.get_room(room_id) {
      Ok(r) => r,
      Err(_) => panic!("Room not found")
    };



    Ok(())
  }

  pub fn clean_up(&self) -> Result<(), Error> {
    let room_ids: Vec<String> = self.rooms.keys().cloned().collect();

    for room_id in room_ids {
      self.remove_room(room_id)?;
    }

    Ok(())
  }

  pub fn remove_room(&self, room_id: String) -> Result<(), Error> {
    // Delete the room entity from the database using the RoomService.
    self.room_service.delete_room(room_id.clone())?;

    // Delete the associated noise keys using the NoiseKeyService.
    self.noise_key_service.delete_key(room_id.clone())?;

    // Remove the room from the SDK's internal state.
    self.rooms.remove(room_id);

    Ok(())
  }

  pub fn get_rooms(&self) -> Result<Vec<Room>, Error> {
    // Use the RoomService to fetch rooms from the database.
    let room_entities = self.room_service.get_rooms()?;

    // Convert RoomEntity instances to Room instances.
    let rooms: Vec<Room> = room_entities
      .iter()
      .map(|room_entity| {
        Room::new(room_entity.room_id.clone(), room_entity.name.clone())
      })
      .collect();

    Ok(rooms)
  }
}

