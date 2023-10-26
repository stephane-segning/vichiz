use libp2p::{gossipsub, mdns, ping, upnp};
use libp2p::swarm::NetworkBehaviour;

#[derive(NetworkBehaviour)]
pub struct AppBehaviour {
  gossip_sub: gossipsub::Behaviour,
  mdns: mdns::tokio::Behaviour,
  upnp: upnp::tokio::Behaviour,
  ping: ping::Behaviour,
}

impl AppBehaviour {
  pub fn new(gossip_sub: gossipsub::Behaviour, mdns: mdns::tokio::Behaviour, upnp: upnp::tokio::Behaviour, ping: ping::Behaviour) -> Self {
    Self { gossip_sub, mdns, upnp, ping }
  }

  pub fn gossip_sub(&mut self) -> &mut gossipsub::Behaviour {
    &mut self.gossip_sub
  }

  pub fn mdns(&mut self) -> &mut mdns::tokio::Behaviour {
    &mut self.mdns
  }

  pub fn upnp(&mut self) -> &mut upnp::tokio::Behaviour {
    &mut self.upnp
  }

  pub fn ping(&mut self) -> &mut ping::Behaviour {
    &mut self.ping
  }
}
