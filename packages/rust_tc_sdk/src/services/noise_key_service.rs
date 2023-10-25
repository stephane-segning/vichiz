use diesel::{QueryResult, RunQueryDsl};
use diesel::row::NamedRow;
use diesel::sqlite::SqliteConnection;
use libp2p::noise::*;
use diesel::r2d2::*;
use libp2p_core::identity;
use rand::rngs::OsRng;

use crate::services::schema::noise_keys::dsl::{noise_keys, id as sql_room_id};
use crate::entities::noise_entity::NoiseKey;

pub struct NoiseKeyService {
  db_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl NoiseKeyService {
  pub fn new(db_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
    NoiseKeyService { db_pool }
  }

  fn generate_rsa_keypair() -> Result<(RSAPrivateKey, String), rsa::errors::Error> {
    // Generate a new RSA key pair with a specific size (e.g., 2048 bits).
    let bits = 2048;
    let private_key = RSAPrivateKey::new(&mut OsRng, bits)?;

    // Convert the private key to a string.
    let private_key_str = private_key.to_pem()?;

    Ok((private_key, private_key_str))
  }

  fn encode_keypair(&self, room_id: &String, keypair: &identity::Keypair) -> Result<NoiseKey, serde_json::Error> {
    let keypair_serde = NoiseKey {
      private: keypair.,
      public: keypair.public().to_vec(),
      id: room_id.clone()
    };
    Ok(keypair_serde)
  }

  fn decode_keypair(&self, keypair_serde: NoiseKey) -> identity::Keypair {
    identity::Keypair::rsa_from_pkcs8(&keypair_serde.private)
  }

  pub fn create_key(&self, room_id: &String) -> Result<(), diesel::result::Error> {
    let identity_keys = identity::Keypair::generate_ed25519();
    let noise_keys = Keypair::<X25519>::new();


    let key = self.encode_keypair(&room_id, &identity_keys)?;
    let mut conn = self.db_pool.get()?;

    diesel::insert_into(noise_keys)
      .values(key)
      .execute(&mut conn)?;

    Ok(())
  }

  pub fn get_key(&self, room_id: &str) -> Result<Keypair<X25519Spec>, diesel::result::Error> {
    let mut conn = self.db_pool.get()?;
    let result: QueryResult<NoiseKey> = noise_keys.filter(sql_room_id.eq(room_id)).first(&mut conn);

    match result {
      Ok(noise_key) => {
        let keypair = self.decode_keypair(noise_key);
        Ok(keypair)
      }
      Err(diesel::result::Error::NotFound) => Err("Key not found".into()),
      Err(err) => Err(err),
    }
  }

  pub fn delete_key(&self, room_id: String) -> Result<(), diesel::result::Error> {
    let mut conn = self.db_pool.get()?;
    diesel::delete(noise_keys.filter(sql_room_id.eq(room_id))).execute(&mut conn)?;

    Ok(())
  }
}
