use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::net::Ipv4Addr;
use libp2p::multiaddr::Protocol::{P2p, QuicV1, Tcp, Udp};
use std::sync::Arc;
use std::time::Duration;

use libp2p::{
    gossipsub,
    identify, identity,
    mdns, Multiaddr,
    noise, ping,
    relay,
    rendezvous, Swarm, tcp,
    yamux,
};
use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;

use crate::entities::room::Room;
use crate::models::behaviour::*;
use crate::models::connection_data::ConnectionData;
use crate::models::error::*;
use crate::services::swarm_controller::ControlMessage;

pub async fn create_private_network(_: Room, config: &ConnectionData, keypair: identity::Keypair) -> Result<Swarm<AppBehaviour>> {
    log::info!("Creating private network");
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_dns()?
        .with_relay_client(noise::Config::new, yamux::Config::default)?
        .with_behaviour(|key, _| {
            // To content-address message, we can take the hash of message and use it as an ID.
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                // TODO check this
                gossipsub::MessageId::from(Vec::from(s.finish().to_be_bytes()))
            };

            // Set a custom gossipsub configuration
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
                .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                .build()?; // Temporary hack because `build` does not return a proper `std::error::Error`.

            // build a gossipsub network behaviour
            let gossip_sub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

            let ping = ping::Behaviour::new(ping::Config::new());

            let identify = identify::Behaviour::new(
                identify::Config::new("/ssegning/1.0.0".into(), key.public()),
            );

            let relay = relay::Behaviour::new(key.public().to_peer_id(), relay::Config::default());

            let rendezvous = rendezvous::client::Behaviour::new(key.clone());

            Ok(AppBehaviour::from((gossip_sub, mdns, ping, identify, relay, rendezvous)))
        })
        .unwrap_or_else(|err| panic!("Failed to build behaviour: {:?}", err))
        .with_swarm_config(|cfg| {
            cfg.with_idle_connection_timeout(
                Duration::from_secs(u64::MAX),
            )
        })
        .build();

    log::info!("Starting private network");

    // Create a Gossipsub topic
    let topic = gossipsub::IdentTopic::new("test-net");

    log::info!("Subscribing to topic: {}", topic);
    // subscribes to our topic
    swarm.behaviour_mut().gossip_sub_mut().subscribe(&topic)?;

    if config.room_listen_on.len() > 0 {
        log::info!("Listening on: {:?}", config.room_listen_on);
        // Tell the swarm to listen on all interfaces and a random, OS-assigned
        // port.
        for url in &config.room_listen_on {
            swarm.listen_on(url.parse()?)?;
        }
    } else {
        // let address_to_listen = Multiaddr::from(Ipv4Addr::UNSPECIFIED);
        let address_to_listen = Multiaddr::from(Ipv4Addr::UNSPECIFIED).with(Tcp(4001));
        log::info!("Will listen on: {:?}", address_to_listen);
        swarm.listen_on(address_to_listen)?;

        let address_to_listen = Multiaddr::from(Ipv4Addr::UNSPECIFIED).with(Tcp(2001)).with(P2p(keypair.public().to_peer_id()));
        log::info!("Will listen on: {:?}", address_to_listen);
        swarm.listen_on(address_to_listen)?;
    }

    if !config.room_multi_address.is_empty() {
        log::info!("Multi-room: Listening on: {:?}", config.room_multi_address);
        for addr in &config.room_multi_address {
            let remote: Multiaddr = addr.parse()?;
            swarm.dial(remote)?;
            log::info!("Dialed {addr}")
        }
    }

    Ok(swarm)
}

pub async fn run_swarm(swarm: Arc<Mutex<Swarm<AppBehaviour>>>, mut receiver: Receiver<ControlMessage>) -> Result<()> {
    log::info!("Running swarm...");
    loop {
        if ControlMessage::Stop == receiver.try_recv()? {
            log::info!("Actually stopping the swarm...");
            break;
        }

        if let Err(e) = process_swarm_events(swarm.clone()).await {
            log::error!("Error while processing swarm events: {:?}", e);
        }
    };

    Ok(())
}

async fn process_swarm_events(swarm: Arc<Mutex<Swarm<AppBehaviour>>>) -> Result<()> {
    log::info!("Processing swarm events...");

    let mut locked_swarm = swarm.lock().await;
    log::info!("Swarm locked");

    match locked_swarm.select_next_some().await {
        SwarmEvent::Behaviour(AppBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
            for (peer_id, _multiaddr) in list {
                log::info!("mDNS discovered a new peer: {peer_id}");
                locked_swarm.behaviour_mut().gossip_sub_mut().add_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
            for (peer_id, _multiaddr) in list {
                log::info!("mDNS discover peer has expired: {peer_id}");
                locked_swarm.behaviour_mut().gossip_sub_mut().remove_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Ping(ping::Event { peer, connection: _, result: Ok(res) })) => {
            log::info!("Ping event from: {:?} in {:?}", peer, res.as_millis());
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Ping(ping::Event { peer, connection: _, result: Err(err) })) => {
            log::info!("Ping failed event from {:?}: {:?}", peer, err);
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::GossipSub(gossipsub::Event::Message { propagation_source: peer_id, message_id: id, message, })) => {
            log::info!("Got message: '{}' with id: {id} from peer: {peer_id}", String::from_utf8_lossy(&message.data));
        }
        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
            log::info!("Connection established with: {peer_id}");
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Rendezvous(rendezvous::client::Event::Discovered { rendezvous_node, .. })) => {
            log::info!("RDV discovered with: {rendezvous_node}");
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Rendezvous(rendezvous::client::Event::DiscoverFailed { rendezvous_node, .. })) => {
            log::info!("RDV discover failed with: {rendezvous_node}");
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Rendezvous(rendezvous::client::Event::Registered { rendezvous_node, .. })) => {
            log::info!("RDV registered with: {rendezvous_node}");
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Rendezvous(rendezvous::client::Event::RegisterFailed { rendezvous_node, .. })) => {
            log::info!("RDV registration failed with: {rendezvous_node}");
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Rendezvous(rendezvous::client::Event::Expired { peer })) => {
            log::info!("RDV expired: {peer}");
        }
        SwarmEvent::ConnectionClosed { peer_id, .. } => {
            log::info!("Connection established with: {peer_id}");
        }
        SwarmEvent::NewListenAddr { address, .. } => {
            log::info!("Local node is listening on {address}");
        }
        event => {
            log::info!("Some other event: {:?}", event);
        }
    };

    Ok(())
}

