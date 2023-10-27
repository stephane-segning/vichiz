use derive_more::From;
use getset::*;
use libp2p::{
    gossipsub,
    identify,
    mdns,
    ping,
    upnp,
};
use libp2p::swarm::*;

#[derive(From, NetworkBehaviour, Getters, MutGetters, Setters)]
pub struct AppBehaviour {
    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    gossip_sub: gossipsub::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    mdns: mdns::tokio::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    upnp: upnp::tokio::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    ping: ping::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    identify: identify::Behaviour,
}