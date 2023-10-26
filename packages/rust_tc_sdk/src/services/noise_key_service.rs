use diesel::{QueryDsl, QueryResult, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use libp2p::identity;
use libp2p::identity::ecdsa;

use diesel::prelude::*;

use crate::services::schema::noise_keys::dsl::{noise_keys, id as sql_room_id};
use crate::entities::noise::NoiseModel;
use crate::models::error::*;

pub struct NoiseKeyService {
  db_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl NoiseKeyService {
  pub fn new(db_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
    NoiseKeyService { db_pool }
  }

  fn generate_ecdsa_keypair(room_id: &str) -> Result<(identity::Keypair, NoiseModel)> {
    let keypair = identity::Keypair::generate_ecdsa();
    let kp = keypair.try_into_ecdsa()?;

    let entity = NoiseModel {
      private: kp.secret().to_bytes(),
      public: kp.public().to_bytes(),
      id: room_id.to_string(),
    };

    Ok((keypair, entity))
  }

  fn from_entity(entity: NoiseModel) -> Result<identity::Keypair> {
    let secret = ecdsa::SecretKey::try_from_bytes(&entity.private)?;
    let keypair = identity::ecdsa::Keypair::from(secret);
    Ok(identity::Keypair::from(keypair))
  }

  pub fn create_key(&self, room_id: &str) -> Result<identity::Keypair> {
    let (keypair, entity) = Self::generate_ecdsa_keypair(room_id)?;

    let mut conn = self.db_pool.get()?;

    diesel::insert_into(noise_keys)
      .values(entity)
      .execute(&mut conn)?;

    Ok(keypair)
  }

  pub fn get_key(&self, room_id: &str) -> Result<identity::Keypair> {
    let conn = &mut self.db_pool.get()?;
    let result: QueryResult<NoiseModel> = noise_keys.filter(sql_room_id.eq(room_id)).first(conn);

    Self::from_entity(result.unwrap())
  }

  pub fn delete_key(&self, room_id: &str) -> Result<()> {
    let mut conn = self.db_pool.get()?;
    diesel::delete(noise_keys.filter(sql_room_id.eq(room_id))).execute(&mut conn)?;

    Ok(())
  }
}
