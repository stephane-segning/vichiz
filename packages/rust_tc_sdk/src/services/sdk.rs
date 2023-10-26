use uuid::Uuid;

use crate::entities::room::Room;
use crate::models::connection_data::ConnectionData;
use crate::models::error::*;
use crate::models::room_option::RoomOption;
use crate::models::rust_sdk_options::*;
use crate::services::connection::establish_connection;
use crate::services::network::{create_private_network, run_swarm};
use crate::services::noise_key_service::NoiseKeyService;
use crate::services::room_service::RoomService;

pub struct RustSDK {
    room_service: RoomService,
    noise_key_service: NoiseKeyService,
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

    pub fn create_room(&mut self, options: RoomOption) -> Result<Room> {
        // Create noise keys for the room.
        let room_id = match options.id {
            None => Uuid::new_v4().to_string(),
            Some(x) => x
        };
        self.noise_key_service.create_key(&room_id)?;

        // Create a Room and persist it.
        let room = Room::new(room_id, options.name);
        self.room_service.create_room(&room)?;

        Ok(room)
    }

    pub async fn start_room(&self, data: ConnectionData) -> Result<()> {
        // Start the room.
        let room = match self.room_service.get_room(&data.room_id) {
            Ok(r) => r,
            Err(_) => panic!("Room not found")
        };

        let keypair = match self.noise_key_service.get_key(&data.room_id) {
            Ok(k) => k,
            Err(_) => panic!("Key not found")
        };

        let swarm = create_private_network(room, &data, keypair).await?;
        let _ = run_swarm(swarm).await;

        Ok(())
    }

    pub fn clean_up(&self) -> Result<()> {
        let room_ids = self.room_service.get_rooms()?;

        for Room { id, .. } in room_ids {
            self.remove_room(&id)?;
        }

        Ok(())
    }

    pub fn remove_room(&self, room_id: &str) -> Result<()> {
        // Delete the room entity from the database using the RoomService.
        self.room_service.delete_room(&room_id)?;

        // Delete the associated noise keys using the NoiseKeyService.
        self.noise_key_service.delete_key(&room_id)?;

        Ok(())
    }

    pub fn get_rooms(&self) -> Result<Vec<Room>> {
        // Use the RoomService to fetch rooms from the database.
        let rooms = self.room_service.get_rooms()?;
        Ok(rooms)
    }
}

