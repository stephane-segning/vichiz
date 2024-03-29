use derive_more::From;
use getset::*;
use libp2p::*;
use libp2p::swarm::*;

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
    rendezvous: rendezvous::client::Behaviour,
}