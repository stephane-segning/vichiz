use std::collections::HashMap;
use std::sync::Arc;

use libp2p::Swarm;
use neon::prelude::*;
use neon_serde3::*;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

use crate::entities::room::Room;
use crate::models::behaviour::AppBehaviour;
use crate::models::callback_payload::CallbackPayload;
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
    channel: Channel,
    room_swarms: HashMap<String, Arc<Mutex<Swarm<AppBehaviour>>>>,
    room_swarm_controller: HashMap<String, SwarmController>,
}

impl RustSDK {
    pub fn new(channel: Channel, options: RustSDKOptions) -> Self {
        log::info!("Initializing Rust SDK");
        let db_pool = establish_connection(options.db_url);

        // Initialize the NoiseKeyService with the connection pool.
        let noise_key_service = NoiseKeyService::new(db_pool.clone());
        let room_service = RoomService::new(db_pool.clone());

        Self {
            noise_key_service,
            room_service,
            channel,
            callbacks: Vec::new(),
            room_swarms: HashMap::new(),
            room_swarm_controller: HashMap::new(),
        }
    }

    pub fn event_occurred<'a, C: Context<'a>>(&self, mut c: C, t: String, data: CallbackPayload) {
        for cb in &self.callbacks {
            let channel = self.channel.clone();
            let cb_clone = cb.clone(&mut c); // Clone the Root handle
            let data_clone = data.clone();
            let t_clone = t.clone();

            // Queue up the JS callback invocation on the V8 main thread
            channel.send(move |mut cx| {
                let callback = cb_clone.to_inner(&mut cx);
                let this = cx.undefined();

                let js_value = to_value(&mut cx, &data_clone)
                    .or_else(|e| cx.throw_error(e.to_string()))
                    .unwrap();

                let js_str = cx.string(&t_clone);

                let args = [js_str.upcast(), js_value.upcast()];
                callback.call(&mut cx, this, args).unwrap();
                Ok(())
            });
        }
    }


    pub fn create_room(&self, options: RoomOption) -> Result<Room> {
        log::info!("Creating room");
        // Create noise keys for the room.
        let room_id = match options.id {
            Some(x) if x.len() > 0 => x,
            _ => Uuid::new_v4().to_string()
        };
        self.noise_key_service.create_key(&room_id)?;

        // Create a Room and persist it.
        let room = Room::from((room_id, options.name));
        self.room_service.create_room(&room)?;

        log::info!("Created room {}", room.id);
        Ok(room)
    }

    pub async fn start_room(&mut self, data: ConnectionData) -> Result<()> {
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
        let (sender, receiver) = mpsc::channel(100);
        let swarm_arc = self.room_swarms.get(&data.room_id).unwrap().clone();
        tokio::spawn(run_swarm(swarm_arc, receiver));
        log::info!("Started swarm controller for room {}", data.room_id);

        let controller = SwarmController { sender };
        self.room_swarm_controller.insert(data.clone().room_id, controller);
        log::info!("Started swarm for room {}", data.room_id);

        log::info!("Started room {}", data.room_id);
        Ok(())
    }

    pub async fn quit_room(&mut self, room_id: &str) -> Result<()> {
        log::info!("Quitting room {}", room_id);
        // Quit the room.
        let _room = self.room_service.get_room(&room_id)
            .expect("Room not found");

        log::info!("Stopping swarm for room {}", room_id);
        if let Some(controller) = self.room_swarm_controller.get_mut(&room_id.to_string()) {
            controller.stop().await?;
        } else {
            log::info!("Swarm for room {} not found", room_id);
        }

        log::info!("Quit room {}", room_id);
        Ok(())
    }

    pub fn register_listener(&mut self, cb: Root<JsFunction>) {
        log::info!("Registering listener");
        self.callbacks.push(cb);
    }

    pub fn clean_up(&self) -> Result<()> {
        log::info!("Cleaning up");
        let room_ids = self.room_service.get_rooms()?;

        for Room { id, .. } in room_ids {
            self.remove_room(&id)?;
        }

        log::info!("Cleaned up");
        Ok(())
    }

    pub fn remove_room(&self, room_id: &str) -> Result<()> {
        log::info!("Removing room {}", room_id);
        // Delete the room entity from the database using the RoomService.
        self.room_service.delete_room(&room_id)?;

        // Delete the associated noise keys using the NoiseKeyService.
        self.noise_key_service.delete_key(&room_id)?;

        log::info!("Removed room {}", room_id);
        Ok(())
    }

    pub fn get_rooms(&self) -> Result<Vec<Room>> {
        log::info!("Getting all rooms");
        // Use the RoomService to fetch rooms from the database.
        let rooms = self.room_service.get_rooms()?;

        log::info!("Got all rooms");
        Ok(rooms)
    }

    pub fn get_room(&self, room_id: &str) -> Result<Room> {
        log::info!("Getting room {}", room_id);
        // Use the RoomService to fetch a room from the database.
        let room = self.room_service.get_room(&room_id)?;

        log::info!("Got room {}", room_id);
        Ok(room)
    }
}

