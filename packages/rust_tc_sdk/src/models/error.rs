use error_chain::error_chain;

error_chain! {
    foreign_links {
        EitherError(libp2p::swarm::derive_prelude::Either<libp2p::tls::certificate::GenError, libp2p::noise::Error>);
        DieselError(diesel::result::Error);
        InfallibleError(std::convert::Infallible);
        DieselR2d2Error(diesel::r2d2::Error);
        R2d2Error(r2d2::Error);
        OtherVariant(libp2p::identity::OtherVariantError);
        Decoding(libp2p::identity::DecodingError);
        NoiseError(libp2p::noise::Error);
        DialError(libp2p::swarm::DialError);
        TlsCertificate(libp2p::tls::certificate::GenError);
        StdError(std::io::Error);
        SubscriptionError(libp2p::gossipsub::SubscriptionError);
        MultiAddrError(libp2p::multiaddr::Error);
        TransportError(libp2p::core::transport::TransportError<std::io::Error>);
        TokioControlMessageError(tokio::sync::mpsc::error::SendError<crate::services::swarm_controller::ControlMessage>);
    }

    errors {
        KeyNotFound {
            description("Key not found")
            display("Key not found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_not_found_error() {
        let error: std::result::Result<String, Error> = Err(Error::from(ErrorKind::KeyNotFound));

        match error {
            Err(Error(ErrorKind::KeyNotFound, _)) => {},
            _ => panic!("Unexpected error type!"),
        }
    }

    // TODO #[test]
    fn test_diesel_error_conversion() {
        // Simulate a Diesel error (this is just an example; you'd use an actual Diesel function
        // that produces an error in a real test):
        let simulated_diesel_error: std::result::Result<String, diesel::result::Error> = Err(diesel::result::Error::NotFound);

        // Convert it to your custom error type:
        let custom_error = simulated_diesel_error.chain_err(|| "failed due to Diesel error");

        // Assert on the error type:
        match custom_error {
            Err(Error(ErrorKind::DieselError(_), _)) => {},
            _ => panic!("Unexpected error type!"),
        }
    }

    // Continue with similar tests for other error types.
}
