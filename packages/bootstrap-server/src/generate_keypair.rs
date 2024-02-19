use libp2p::identity;
use libp2p::identity::Keypair;
use serde_json::{from_reader, to_writer};
use tokio::fs::{File, OpenOptions};

use crate::error::*;
use crate::kp_wrapper::KeypairWrapper;

pub async fn generate_keypair(keypair_path: &str) -> Result<Keypair> {
    match File::open(keypair_path).await {
        Ok(file) => get_keypair(file).await,
        Err(_) => create_keypair(&keypair_path).await,
    }
}

async fn create_keypair(keypair_path: &&str) -> Result<Keypair> {
    let new_keypair = identity::Keypair::generate_ecdsa();
    let kp = new_keypair.clone().try_into_ecdsa()?;
    let wrapper = KeypairWrapper::new(kp.secret().to_bytes(), kp.public().to_bytes());

    let file = OpenOptions::new().write(true).create_new(true).open(&keypair_path).await?;
    to_writer(file.into_std().await, &wrapper)?;

    log::info!("Generated new keypair and saved it to file");
    Ok(new_keypair)
}

async fn get_keypair(file: File) -> Result<Keypair> {
    let wrapper: KeypairWrapper = from_reader(file.into_std().await)?;
    let secret = identity::ecdsa::SecretKey::try_from_bytes(&wrapper.secret())?;
    let kp = identity::ecdsa::Keypair::from(secret);

    log::info!("Loaded keypair from file");
    Ok(identity::Keypair::from(kp))
}