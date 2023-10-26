use error_chain::error_chain;

// fn helper() -> PoisonError<> {
//
// }


error_chain! {
    foreign_links {
        // NeonError(neon::result::Throw);
        // PoisonError(std::sync::PoisonError);
        EitherError(libp2p::swarm::derive_prelude::Either<libp2p::tls::certificate::GenError, libp2p::noise::Error>);
        DieselError(diesel::result::Error);
        DieselR2d2Error(diesel::r2d2::Error);
        R2d2Error(r2d2::Error);
        OtherVariant(libp2p::identity::OtherVariantError);
        Decoding(libp2p::identity::DecodingError);
        NoiseError(libp2p::noise::Error);
        DialError(libp2p::swarm::DialError);
        TlsCertificate(libp2p::tls::certificate::GenError);
        StdError(std::io::Error);
        // BehaviourError(libp2p::builder::phase::behaviour::BehaviourError);
        SubscriptionError(libp2p::gossipsub::SubscriptionError);
        MultiAddrError(libp2p::multiaddr::Error);
        TransportError(libp2p::core::transport::TransportError<std::io::Error>);
    }

    errors {
        KeyNotFound {
            description("Key not found")
            display("Key not found")
        }
    }
}
