use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;

use futures::{FutureExt, StreamExt, TryFutureExt, TryStreamExt};
use libp2p::{
    gossipsub,
    identify, identity,
    mdns, Multiaddr,
    noise, ping,
    relay,
    Swarm, tcp,
    upnp, yamux,
};
use libp2p::multiaddr::Protocol;
use libp2p::swarm::SwarmEvent;
use libp2p_webrtc;
use libp2p_webrtc::tokio::Certificate;
use rand::thread_rng;
use tokio::sync::{mpsc, Mutex};

use crate::entities::room::Room;
use crate::models::behaviour::*;
use crate::models::connection_data::ConnectionData;
use crate::models::error::*;
use crate::services::swarm_controller::ControlMessage;

#[inline]
pub async fn create_private_network(_: Room, config: &ConnectionData, keypair: identity::Keypair) -> Result<Swarm<AppBehaviour>> {
    log::info!("Creating private network");
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_other_transport(|key| {
            libp2p_webrtc::tokio::Transport::new(key.clone(), Certificate::generate(&mut thread_rng()).unwrap())
        })?
        .with_dns()?
        .with_relay_client(noise::Config::new, yamux::Config::default)?
        .with_behaviour(|key, _| {
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
                .map_err(|msg| tokio::io::Error::new(tokio::io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

            // build a gossipsub network behaviour
            let gossip_sub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

            let upnp = upnp::tokio::Behaviour::from(upnp::tokio::Behaviour::default());

            let ping = ping::Behaviour::new(ping::Config::new());

            let identify = identify::Behaviour::new(
                identify::Config::new("/ipfs/0.1.0".into(), key.public()),
            );

            let relay = relay::Behaviour::new(key.public().to_peer_id(), relay::Config::default());

            Ok(AppBehaviour::from((gossip_sub, mdns, upnp, ping, identify, relay)))
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
        let address_webrtc = Multiaddr::from(Ipv4Addr::UNSPECIFIED)
            .with(Protocol::Tcp(21000));
        log::info!("Will listen on: {:?}", address_webrtc);
        swarm.listen_on(address_webrtc)?;

        // let address_webrtc = Multiaddr::from(Ipv4Addr::UNSPECIFIED)
        //     .with(Protocol::Ws("ws-relay".into()));
        // log::info!("Will listen on: {:?}", address_webrtc);
        // swarm.listen_on(address_webrtc)?;

        let address_webrtc = Multiaddr::from(Ipv4Addr::UNSPECIFIED)
            .with(Protocol::Udp(21000))
            .with(Protocol::QuicV1);
        log::info!("Will listen on: {:?}", address_webrtc);
        swarm.listen_on(address_webrtc)?;
    }

    if !config.room_multi_address.is_empty() {
        log::info!("Multiroom: Listening on: {:?}", config.room_multi_address);
        for addr in &config.room_multi_address {
            let remote: Multiaddr = addr.parse()?;
            swarm.dial(remote)?;
            log::info!("Dialed {addr}")
        }
    }

    Ok(swarm)
}

#[inline]
pub async fn run_swarm(swarm: Arc<Mutex<Swarm<AppBehaviour>>>, mut receiver: mpsc::Receiver<ControlMessage>) {
    log::info!("Running swarm...");
    loop {
        tokio::select! {
            message = receiver.recv() => {
                if let Some(ControlMessage::Stop) = message {
                    log::info!("Actually stopping the swarm...");
                    break;
                }
                // You can handle other messages here if needed
            }
            _ = tokio::spawn(process_swarm_events(swarm.clone())) => {}
        }
    }
}

async fn process_swarm_events(swarm: Arc<Mutex<Swarm<AppBehaviour>>>) -> Result<()> {
    log::info!("Processing swarm events...");

    let mut locked_swarm = swarm.lock().await;
    log::info!("Swarm locked");

    let swarm_event = locked_swarm.select_next_some().await;
    log::info!("Swarm select_next_some");

    // Handle the swarm_event as before:
    match swarm_event {
        SwarmEvent::Behaviour(AppBehaviourEvent::Upnp(upnp::Event::NewExternalAddr(addr))) => {
            log::info!("New external address: {addr}");
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Upnp(upnp::Event::ExpiredExternalAddr(addr))) => {
            log::info!("Expired external address: {addr}");
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Upnp(upnp::Event::GatewayNotFound)) => {
            log::info!("Gateway does not support UPnP");
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Upnp(upnp::Event::NonRoutableGateway)) => {
            log::info!("Gateway is not exposed directly to the public Internet, i.e. it itself has a private IP address.");
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
            for (peer_id, _multiaddr) in list {
                log::info!("mDNS discovered a new peer: {peer_id}");
                swarm.lock().await.behaviour_mut().gossip_sub_mut().add_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
            for (peer_id, _multiaddr) in list {
                log::info!("mDNS discover peer has expired: {peer_id}");
                swarm.lock().await.behaviour_mut().gossip_sub_mut().remove_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::Ping(ping::Event { peer, connection: _, result: _ })) => {
            log::info!("Ping event from: {:?}", peer);
        }
        SwarmEvent::Behaviour(AppBehaviourEvent::GossipSub(gossipsub::Event::Message { propagation_source: peer_id, message_id: id, message, })) => log::info!(
            "Got message: '{}' with id: {id} from peer: {peer_id}",
            String::from_utf8_lossy(&message.data),
        ),
        SwarmEvent::NewListenAddr { address, .. } => {
            log::info!("Local node is listening on {address}");
        }
        event => {
            log::info!("Some other event: {:?}", event);
        }
    };

    Ok(())
}

