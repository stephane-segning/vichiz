use derive_more::*;
use getset::*;
use serde::*;

#[derive(From, Getters, MutGetters, Setters, Serialize, Debug, Deserialize, Clone)]
pub struct CallbackPayload {
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
    pub data: String,
}
