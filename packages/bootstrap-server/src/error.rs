use error_chain::error_chain;
use libp2p::core::transport::TransportError as BaseTransportError;
use libp2p::noise::Error as BaseNoiseError;
use libp2p::swarm::derive_prelude::*;
use libp2p::tls::certificate::GenError;

error_chain! {
    foreign_links {
        R2d2Error(r2d2::Error);
        EitherError(Either<GenError, BaseNoiseError>);
        InfallibleError(std::convert::Infallible);
        OtherVariant(libp2p::identity::OtherVariantError);
        Decoding(libp2p::identity::DecodingError);
        NoiseError(BaseNoiseError);
        DialError(libp2p::swarm::DialError);
        TlsCertificate(libp2p::tls::certificate::GenError);
        StdError(std::io::Error);
        SubscriptionError(libp2p::gossipsub::SubscriptionError);
        MultiAddrError(libp2p::multiaddr::Error);
        TransportError(BaseTransportError<std::io::Error>);
        SerdeJsonError(serde_json::Error);
        DieselError(diesel::result::Error);
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
            Err(Error(ErrorKind::KeyNotFound, _)) => {}
            _ => panic!("Unexpected error type!"),
        }
    }

    #[test]
    fn test_diesel_error_conversion() {
        // Simulate a Diesel error (this is just an example; you'd use an actual Diesel function
        // that produces an error in a real test):
        let simulated_diesel_error: std::result::Result<String, BaseNoiseError::InvalidLength> = Err(BaseNoiseError::InvalidLength);

        // Convert it to your custom error type:
        let custom_error = simulated_diesel_error.chain_err(|| "failed due to Diesel error");

        // Assert on the error type:
        match custom_error {
            Err(Error(ErrorKind::NoiseError(_), _)) => {}
            _ => panic!("Unexpected error type!"),
        }
    }

    // Continue with similar tests for other error types.
}
