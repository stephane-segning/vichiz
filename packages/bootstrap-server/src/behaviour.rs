use derive_more::From;
use getset::*;
use libp2p::*;
use libp2p::swarm::*;
use serde_derive::{Deserialize, Serialize};

use crate::record_store::RedisRecordStore;

#[derive(From, NetworkBehaviour, Getters, MutGetters, Setters)]
pub struct AppBehaviour {
    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    gossip_sub: gossipsub::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    mdns: mdns::tokio::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    ping: ping::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    identify: identify::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    relay: relay::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    rendezvous: rendezvous::server::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    kad: kad::Behaviour<RedisRecordStore>,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    request_response: request_response::cbor::Behaviour<FileRequest, FileResponse>,
}

// Simple file exchange protocol
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileRequest(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileResponse(Vec<u8>);