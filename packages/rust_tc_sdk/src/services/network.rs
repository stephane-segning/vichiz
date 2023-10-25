use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use futures::stream::StreamExt;
use libp2p::{gossipsub, mdns, Multiaddr, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, upnp, yamux};
use tokio::io;

#[derive(NetworkBehaviour)]
struct MyBehaviour {
  gossip_sub: gossipsub::Behaviour,
  mdns: mdns::tokio::Behaviour,
}

pub struct PrivateNetworkConfig {
  pub addresses: Vec<String>,
  pub listen_on: Vec<String>,
}

async fn create_private_network(config: PrivateNetworkConfig) -> Result<(), Box<dyn Error>> {

  // Create a random PeerId
  println!("Local peer id: {peer_id:?}");

  let mut swarm = libp2p::SwarmBuilder::with_new_identity()
    .with_tokio()
    .with_tcp(
      tcp::Config::default(),
      (libp2p_tls::Config::new, libp2p_noise::Config::new),
      (yamux::Config::default, libp2p_mplex::MplexConfig::new),
    )?
    .with_dns()?
    .with_quic()
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

      Ok(MyBehaviour { gossip_sub, mdns })
    })?
    .build();

  // Create a Gossipsub topic
  let topic = gossipsub::IdentTopic::new("test-net");
  // subscribes to our topic
  swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

  // Tell the swarm to listen on all interfaces and a random, OS-assigned
  // port.
  for url in config.listen_on {
    swarm.listen_on(url.parse()?)?;
  }

  for addr in config.addresses {
    let remote: Multiaddr = addr.parse()?;
    swarm.dial(remote)?;
    println!("Dialed {addr}")
  }

  loop {
    match swarm.select_next_some().await {
      SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
      SwarmEvent::Behaviour(upnp::Event::NewExternalAddr(addr)) => {
        println!("New external address: {addr}");
      }
      SwarmEvent::Behaviour(upnp::Event::GatewayNotFound) => {
        println!("Gateway does not support UPnP");
        break;
      }
      SwarmEvent::Behaviour(upnp::Event::NonRoutableGateway) => {
        println!("Gateway is not exposed directly to the public Internet, i.e. it itself has a private IP address.");
        break;
      }
      SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
        for (peer_id, _multiaddr) in list {
          println!("mDNS discovered a new peer: {peer_id}");
          swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
        }
      }
      SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
        for (peer_id, _multiaddr) in list {
          println!("mDNS discover peer has expired: {peer_id}");
          swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
        }
      }
      SwarmEvent::Behaviour(MyBehaviourEvent::GossipSub(gossipsub::Event::Message {
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
      _ => {}
    }
  }

  Ok(())
}

