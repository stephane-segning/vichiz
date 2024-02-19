use getset::{Getters, MutGetters, Setters};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, MutGetters, Setters, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct KeypairWrapper {
    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    secret: Vec<u8>,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    public: Vec<u8>,
}

impl KeypairWrapper {
    pub(crate) fn new(secret: Vec<u8>, public: Vec<u8>) -> Self {
        Self { secret, public }
    }
}