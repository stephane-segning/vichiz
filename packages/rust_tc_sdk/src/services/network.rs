use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use futures::StreamExt;
use libp2p::{
  gossipsub,
  identity, mdns,
  Multiaddr, noise,
  ping, Swarm,
  swarm::SwarmEvent,
  tcp, tls, upnp,
  yamux,
};
use tokio::io;

use crate::entities::room::Room;
use crate::models::behaviour::*;
use crate::models::connection_data::ConnectionData;
use crate::models::error::*;

#[inline]
pub async fn create_private_network(_: Room, config: &ConnectionData, keypair: identity::Keypair) -> Result<Swarm<AppBehaviour>> {
  let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
    .with_tokio()
    .with_tcp(
      tcp::Config::default(),
      (tls::Config::new, noise::Config::new),
      yamux::Config::default,
    )?
    .with_quic()
    .with_dns()?
    .with_behaviour(|key| {
      // To content-address message, we can take the hash of message and use it as an ID.
      let message_id_fn = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
      };

      // Set a custom gossipsub configuration
      let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
        .build()
        .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

      // build a gossipsub network behaviour
      let gossip_sub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(key.clone()),
        gossipsub_config,
      )?;

      let mdns =
        mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

      let upnp = upnp::tokio::Behaviour::from(upnp::tokio::Behaviour::default());

      let ping = ping::Behaviour::new(ping::Config::new());

      Ok(AppBehaviour::new(gossip_sub, mdns, upnp, ping))
    })
    .unwrap_or_else(|err| panic!("Failed to build behaviour: {:?}", err))
    .build();

  // Create a Gossipsub topic
  let topic = gossipsub::IdentTopic::new("test-net");
  // subscribes to our topic
  swarm.behaviour_mut().gossip_sub().subscribe(&topic)?;

  // Tell the swarm to listen on all interfaces and a random, OS-assigned
  // port.
  for url in &config.room_listen_on {
    swarm.listen_on(url.parse()?)?;
  }

  for addr in &config.room_multiaddress {
    let remote: Multiaddr = addr.parse()?;
    swarm.dial(remote)?;
    println!("Dialed {addr}")
  }

  Ok(swarm)
}

#[inline]
pub async fn run_swarm(mut swarm: Swarm<AppBehaviour>) -> Result<()> {
  loop {
    match swarm.select_next_some().await {
      SwarmEvent::Behaviour(AppBehaviourEvent::Upnp(upnp::Event::NewExternalAddr(addr))) => {
        println!("New external address: {addr}");
      }
      SwarmEvent::Behaviour(AppBehaviourEvent::Upnp(upnp::Event::ExpiredExternalAddr(addr))) => {
        println!("Expired external address: {addr}");
      }
      SwarmEvent::Behaviour(AppBehaviourEvent::Upnp(upnp::Event::GatewayNotFound)) => {
        println!("Gateway does not support UPnP");
        break;
      }
      SwarmEvent::Behaviour(AppBehaviourEvent::Upnp(upnp::Event::NonRoutableGateway)) => {
        println!("Gateway is not exposed directly to the public Internet, i.e. it itself has a private IP address.");
        break;
      }
      SwarmEvent::Behaviour(AppBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
        for (peer_id, _multiaddr) in list {
          println!("mDNS discovered a new peer: {peer_id}");
          swarm.behaviour_mut().gossip_sub().add_explicit_peer(&peer_id);
        }
      }
      SwarmEvent::Behaviour(AppBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
        for (peer_id, _multiaddr) in list {
          println!("mDNS discover peer has expired: {peer_id}");
          swarm.behaviour_mut().gossip_sub().remove_explicit_peer(&peer_id);
        }
      }
      SwarmEvent::Behaviour(AppBehaviourEvent::Ping(ping::Event { peer, connection: _, result: _ })) => {
        println!("Ping event from: {:?}", peer);
      }
      SwarmEvent::Behaviour(AppBehaviourEvent::GossipSub(gossipsub::Event::Message {
                                                           propagation_source: peer_id,
                                                           message_id: id,
                                                           message,
                                                         })) => println!(
        "Got message: '{}' with id: {id} from peer: {peer_id}",
        String::from_utf8_lossy(&message.data),
      ),
      SwarmEvent::NewListenAddr { address, .. } => {
        println!("Local node is listening on {address}");
      }
      event => {
        println!("Some other event: {:?}", event);
      }
    }
  }

  Ok(())
}
