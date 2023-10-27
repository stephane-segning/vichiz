use derive_more::From;
use getset::*;
use serde::*;

#[derive(From, Getters, MutGetters, Setters, Serialize, Debug, Deserialize)]
pub struct RoomId {
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
    pub id: String,
}
