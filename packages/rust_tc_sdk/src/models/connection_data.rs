use getset::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, MutGetters, Setters, Clone, Debug)]
pub struct ConnectionData {
  #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
  pub room_id: String,

  #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
  pub room_multi_address: Vec<String>,

  #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
  pub room_listen_on: Vec<String>,
}
