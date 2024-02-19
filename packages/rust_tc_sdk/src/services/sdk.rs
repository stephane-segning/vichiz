use std::collections::HashMap;
use std::sync::Arc;
use std::thread::spawn;

use libp2p::Swarm;
use neon::prelude::*;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::entities::room::Room;
use crate::models::behaviour::AppBehaviour;
use crate::models::connection_data::ConnectionData;
use crate::models::error::*;
use crate::models::room_option::RoomOption;
use crate::models::rust_sdk_options::*;
use crate::services::connection::establish_connection;
use crate::services::network::{create_private_network, run_swarm};
use crate::services::noise_key_service::NoiseKeyService;
use crate::services::room_service::RoomService;
use crate::services::swarm_controller::SwarmController;

pub struct RustSDK {
    room_service: RoomService,
    noise_key_service: NoiseKeyService,
    callbacks: Vec<Root<JsFunction>>,
    room_swarms: HashMap<String, Arc<Mutex<Swarm<AppBehaviour>>>>,
    room_swarm_controller: HashMap<String, SwarmController>,
}

impl RustSDK {
    pub fn new(options: RustSDKOptions) -> Self {
        log::info!("Initializing Rust SDK");
        let db_pool = establish_connection(options.db_url);

        // Initialize the NoiseKeyService with the connection pool.
        let noise_key_service = NoiseKeyService::new(db_pool.clone());
        let room_service = RoomService::new(db_pool.clone());

        Self {
            noise_key_service,
            room_service,
            callbacks: Vec::new(),
            room_swarms: HashMap::new(),
            room_swarm_controller: HashMap::new(),
        }
    }

    pub async fn create_room(&self, options: RoomOption) -> Result<Room> {
        log::info!("Creating room");
        // Create noise keys for the room.
        let room_id = match options.id {
            Some(x) if x.len() > 0 => x,
            _ => Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"tc.ssegning.com").to_string()
        };
        self.noise_key_service.create_key(&room_id)?;

        // Create a Room and persist it.
        let room = Room::from((room_id, options.name));
        self.room_service.create_room(&room)?;

        log::info!("Created room {}", room.id);
        Ok(room)
    }

    pub async fn start_room(&mut self, data: ConnectionData) -> Result<()> {
        let swarm_arc = self.room_swarms.contains_key(&data.room_id);
        if swarm_arc {
            log::info!("Swarm for room {} already exists", data.room_id);
            return Ok(());
        }

        log::info!("Starting room {}", data.room_id);
        // Start the room.
        let room = match self.room_service.get_room(&data.room_id) {
            Ok(r) => r,
            Err(_) => panic!("Room not found")
        };

        let keypair = match self.noise_key_service.get_key(&data.room_id) {
            Ok(k) => k,
            Err(_) => panic!("Key not found")
        };

        log::info!("Starting swarm for room {}", data.room_id);
        let swarm: Swarm<AppBehaviour> = create_private_network(room, &data, keypair).await?;
        self.room_swarms.insert(data.clone().room_id, Arc::new(Mutex::new(swarm)));

        log::info!("Starting swarm controller for room {}", data.room_id);
        let (sender, receiver) = mpsc::channel(8196);
        let swarm_arc = self.room_swarms.get(&data.room_id).unwrap().clone();
        spawn(|| run_swarm(swarm_arc, receiver));
        log::info!("Started swarm controller for room {}", data.room_id);

        let controller = SwarmController { sender };
        self.room_swarm_controller.insert(data.clone().room_id, controller);
        log::info!("Started swarm for room {}", data.room_id);

        log::info!("Started room {}", data.room_id);
        Ok(())
    }

    async fn handle_swarm_event(&self) {}

    pub async fn quit_room(&mut self, room_id: &str) -> Result<()> {
        log::info!("Quitting room {}", room_id);
        // Quit the room.
        let _room = self.room_service.get_room(&room_id)
            .expect("Room not found");

        log::info!("Stopping swarm for room {}", room_id);
        if let Some(controller) = self.room_swarm_controller.get_mut(&room_id.to_string()) {
            controller.stop().await;
        } else {
            log::info!("Swarm for room {} not found", room_id);
        }

        log::info!("Stopping swarm for room {}", room_id);
        if let Some(mutex) = self.room_swarms.get_mut(&room_id.to_string()) {
            let swarm = mutex.lock().await;
            drop(swarm);
        } else {
            log::info!("Swarm for room {} not found", room_id);
        }

        log::info!("Removing swarm for room {}", room_id);
        self.room_swarms.remove(&room_id.to_string());

        log::info!("Quit room {}", room_id);
        Ok(())
    }

    pub async fn register_listener(&mut self, cb: Root<JsFunction>) {
        log::info!("Registering listener");
        self.callbacks.push(cb);
    }

    pub async fn clean_up(&self) -> Result<()> {
        log::info!("Cleaning up");
        let room_ids = self.room_service.get_rooms()?;

        for Room { id, .. } in room_ids {
            self.remove_room(&id).await?;
        }

        log::info!("Cleaned up");
        Ok(())
    }

    pub async fn remove_room(&self, room_id: &str) -> Result<()> {
        log::info!("Removing room {}", room_id);
        // Delete the room entity from the database using the RoomService.
        self.room_service.delete_room(&room_id)?;

        // Delete the associated noise keys using the NoiseKeyService.
        self.noise_key_service.delete_key(&room_id)?;

        log::info!("Removed room {}", room_id);
        Ok(())
    }

    pub async fn get_rooms(&self) -> Result<Vec<Room>> {
        log::info!("Getting all rooms");
        // Use the RoomService to fetch rooms from the database.
        let rooms = self.room_service.get_rooms()?;

        log::info!("Got all rooms");
        Ok(rooms)
    }

    pub async fn get_room(&self, room_id: &str) -> Result<Room> {
        log::info!("Getting room {}", room_id);
        // Use the RoomService to fetch a room from the database.
        let room = self.room_service.get_room(&room_id)?;

        log::info!("Got room {}", room_id);
        Ok(room)
    }
}

