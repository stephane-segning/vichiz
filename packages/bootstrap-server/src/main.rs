use std::net::Ipv4Addr;
use std::time::Duration;

use libp2p::{gossipsub, identify, kad, mdns, Multiaddr, noise, ping, relay, rendezvous, request_response, StreamProtocol, SwarmBuilder, tcp, tls, yamux};
use libp2p::futures::StreamExt;
use libp2p::multiaddr::Protocol::{QuicV1, Tcp, Udp, P2p};
use libp2p::request_response::ProtocolSupport;
use libp2p::swarm::SwarmEvent;

use crate::behaviour::{AppBehaviour, AppBehaviourEvent};
use crate::error::Result;
use crate::establish_connection::establish_connection;
use crate::generate_keypair::generate_keypair;
use crate::record_store::RedisRecordStore;

mod error;
mod behaviour;
mod kp_wrapper;
mod record_store;
mod establish_connection;
mod record_models;
mod schema;
mod provider_models;
mod generate_keypair;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Create a connection pool to the database
    let redis_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "redis://localhost:6379".into());
    let pool = establish_connection(&redis_url).await;
    let store = RedisRecordStore::from(pool.clone());

    // Generate identity keypair
    let keypair_path = std::env::var("KEYPAIR_PATH").unwrap_or_else(|_| "./target/key_pair.json".into());
    let id_keys = generate_keypair(&keypair_path).await?;

    // Create a PeerId from the public key
    let peer_id = id_keys.public().to_peer_id();
    log::info!("Peer ID: {}", peer_id.to_string());

    // Create a Swarm to manage peers and events
    let mut swarm = SwarmBuilder::with_existing_identity(id_keys)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            (tls::Config::new, noise::Config::new),
            yamux::Config::default,
        )?
        .with_quic()
        .with_dns()?
        .with_behaviour(|key| {
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
                .build()
                .unwrap();

            let gossip_sub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            ).unwrap();

            let mdns_config = mdns::Config {
                ttl: Duration::from_secs(6 * 60),
                query_interval: Duration::from_secs(5 * 60),
                enable_ipv6: true,
            };
            let mdns_behaviour =
                mdns::tokio::Behaviour::new(mdns_config, key.public().to_peer_id())
                    .unwrap();

            let ping_behaviour = ping::Behaviour::new(ping::Config::new());

            let identify_behaviour = identify::Behaviour::new(
                identify::Config::new("/ssegning/1.0.0".into(), key.public()),
            );

            let relay_behaviour = relay::Behaviour::new(key.public().to_peer_id(), relay::Config::default());

            let rdv_config = rendezvous::server::Config::default()
                .with_max_ttl(5 * 60)
                .with_min_ttl(60);
            let rendezvous_behaviour = rendezvous::server::Behaviour::new(rdv_config);

            // Create a Kademlia behaviour.
            let mut kad_cfg = kad::Config::default();
            kad_cfg.set_query_timeout(Duration::from_secs(5 * 60));
            let mut kad_behaviour = kad::Behaviour::with_config(key.public().to_peer_id(), store, kad_cfg);
            kad_behaviour.set_mode(Some(kad::Mode::Server));

            let request_response_behaviour = request_response::cbor::Behaviour::new(
                [(
                    StreamProtocol::new("/file-exchange/1"),
                    ProtocolSupport::Full,
                )],
                request_response::Config::default(),
            );

            return AppBehaviour::from((gossip_sub, mdns_behaviour, ping_behaviour, identify_behaviour, relay_behaviour, rendezvous_behaviour, kad_behaviour, request_response_behaviour));
        })?
        .build();

    let address_to_listen = Multiaddr::from(Ipv4Addr::UNSPECIFIED).with(Tcp(4000));
    log::info!("Will listen on: {:?}", address_to_listen);
    swarm.listen_on(address_to_listen)?;

    let address_to_listen = Multiaddr::from(Ipv4Addr::UNSPECIFIED).with(Udp(4000)).with(QuicV1);
    log::info!("Will listen on: {:?}", address_to_listen);
    swarm.listen_on(address_to_listen)?;

    let address_to_listen = Multiaddr::from(Ipv4Addr::UNSPECIFIED).with(Tcp(2000)).with(P2p(peer_id));
    log::info!("Will listen on: {:?}", address_to_listen);
    swarm.listen_on(address_to_listen)?;


    // Start the event loop
    loop {
        let event = swarm.select_next_some().await;

        match event {
            SwarmEvent::Behaviour(AppBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, _multiaddr) in list {
                    log::info!("mDNS discovered a new peer: {peer_id}");
                    swarm.behaviour_mut().gossip_sub_mut().add_explicit_peer(&peer_id);
                }
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                for (peer_id, _multiaddr) in list {
                    log::info!("mDNS discover peer has expired: {peer_id}");
                    swarm.behaviour_mut().gossip_sub_mut().remove_explicit_peer(&peer_id);
                }
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Ping(ping::Event { peer, connection: _, result: Ok(res) })) => {
                log::info!("Ping event from: {:?} in {:?}", peer, res.as_millis());
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Ping(ping::Event { peer, connection: _, result: Err(err) })) => {
                log::info!("Ping failed event from {:?}: {:?}", peer, err);
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::GossipSub(gossipsub::Event::Message { propagation_source: peer_id, message_id: id, message, })) => {
                log::info!("GossipSub message: '{}' with id: {id} from peer: {peer_id}", String::from_utf8_lossy(&message.data));
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::GossipSub(gossipsub::Event::GossipsubNotSupported { .. })) => {
                log::info!("GossipSub not supported");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::GossipSub(gossipsub::Event::Subscribed { .. })) => {
                log::info!("GossipSub subscribed");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::GossipSub(gossipsub::Event::Unsubscribed { .. })) => {
                log::info!("GossipSub unsubscribed");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Rendezvous(rendezvous::server::Event::DiscoverServed { .. })) => {
                log::info!("RDV discover served");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Rendezvous(rendezvous::server::Event::DiscoverNotServed { .. })) => {
                log::info!("RDV discover not served")
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Rendezvous(rendezvous::server::Event::PeerRegistered { .. })) => {
                log::info!("RDV peer registered");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Rendezvous(rendezvous::server::Event::PeerNotRegistered { .. })) => {
                log::info!("RDV peer-not-registered");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Rendezvous(rendezvous::server::Event::RegistrationExpired { .. })) => {
                log::info!("RDV registration-expired");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Kad(kad::Event::InboundRequest { request })) => {
                log::info!("KAD inbound-request: {:?}", request);
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Kad(kad::Event::OutboundQueryProgressed { .. })) => {
                log::info!("KAD outbound-query-progressed");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Kad(kad::Event::PendingRoutablePeer { .. })) => {
                log::info!("KAD pending-routable-peer");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Kad(kad::Event::RoutablePeer { .. })) => {
                log::info!("KAD pending-routable-peer");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Identify(identify::Event::Received { info, peer_id })) => {
                log::info!("Identity received {peer_id}");

                // TODO: Add the observed address to the peerstore
                swarm.add_external_address(info.observed_addr);
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Identify(identify::Event::Sent { .. })) => {
                log::info!("Identity sent");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Identify(identify::Event::Pushed { peer_id })) => {
                log::info!("Identity pushed {peer_id}");
            }
            SwarmEvent::Behaviour(AppBehaviourEvent::Identify(identify::Event::Error { peer_id, error })) => {
                log::info!("Identity {peer_id} error: {:?}", error);
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                log::info!("Connection established with: {peer_id}");
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                log::info!("Connection established with: {peer_id}");
            }
            SwarmEvent::IncomingConnection { .. } => {
                log::info!("Incoming connection");
            }
            SwarmEvent::IncomingConnectionError { .. } => {
                log::info!("Incoming connection error");
            }
            SwarmEvent::OutgoingConnectionError { .. } => {
                log::info!("Outgoing connection error");
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                log::info!("Local node is listening on {address}");
            }
            SwarmEvent::ExpiredListenAddr { .. } => {
                log::info!("Expired listen address");
            }
            SwarmEvent::ListenerClosed { .. } => {
                log::info!("Listener closed");
            }
            SwarmEvent::ListenerError { .. } => {
                log::info!("Listener error");
            }
            SwarmEvent::Dialing { .. } => {
                log::info!("Dialing ...");
            }
            event => {
                log::info!("Some other event: {:?}", event);
            }
        }
    }
}
