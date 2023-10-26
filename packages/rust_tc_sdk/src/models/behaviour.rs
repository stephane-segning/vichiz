use getset::MutGetters;
use libp2p::{gossipsub, mdns, ping, upnp};
use libp2p::swarm::NetworkBehaviour;

#[derive(NetworkBehaviour, MutGetters)]
pub struct AppBehaviour {
    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    gossip_sub: gossipsub::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    mdns: mdns::tokio::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    upnp: upnp::tokio::Behaviour,

    #[getset(get_copy = "pub", get_mut = "pub", get = "pub")]
    ping: ping::Behaviour,
}

impl AppBehaviour {
    pub fn new(gossip_sub: gossipsub::Behaviour, mdns: mdns::tokio::Behaviour, upnp: upnp::tokio::Behaviour, ping: ping::Behaviour) -> Self {
        Self { gossip_sub, mdns, upnp, ping }
    }
}
