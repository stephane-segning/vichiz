use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConnectionData {
  pub room_id: String,
  pub room_public_key: Vec<u8>,
  pub room_multiaddress: Vec<String>,
  pub room_listen_on: Vec<String>,
}
