use diesel::{QueryDsl, QueryResult, RunQueryDsl};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use libp2p::identity;
use libp2p::identity::ecdsa;

use crate::entities::noise::NoiseModel;
use crate::models::error::*;
use crate::schema::noise_keys::dsl::*;

#[derive(Debug)]
pub struct NoiseKeyService {
    db_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl NoiseKeyService {
    pub fn new(db_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        NoiseKeyService { db_pool }
    }

    fn generate_ecdsa_keypair(room_id: &str) -> Result<NoiseModel> {
        log::info!("Generating ECDSA keypair for room {}", room_id);
        let keypair = identity::Keypair::generate_ecdsa();
        let kp = keypair.try_into_ecdsa()?;

        let entity = NoiseModel::from((room_id.to_string(), kp.secret().to_bytes(), kp.public().to_bytes()));

        log::info!("Generated ECDSA keypair for room {}", room_id);
        Ok(entity)
    }

    #[inline]
    fn from_entity(entity: NoiseModel) -> Result<identity::Keypair> {
        log::info!("Converting NoiseModel to identity::Keypair");
        let secret = ecdsa::SecretKey::try_from_bytes(&entity.private)?;
        let keypair = identity::ecdsa::Keypair::from(secret);

        log::info!("Converted NoiseModel to identity::Keypair");
        Ok(identity::Keypair::from(keypair))
    }

    pub fn create_key(&self, room_id: &str) -> Result<()> {
        log::info!("Creating ECDSA keypair for room {}", room_id);
        let entity = Self::generate_ecdsa_keypair(room_id)?;

        let mut conn = self.db_pool.get()?;

        diesel::insert_into(noise_keys)
            .values(entity)
            .execute(&mut conn)?;

        log::info!("Created ECDSA keypair for room {}", room_id);
        Ok(())
    }

    pub fn get_key(&self, room_id: &str) -> Result<identity::Keypair> {
        log::info!("Getting ECDSA keypair for room {}", room_id);
        let conn = &mut self.db_pool.get()?;
        let result: QueryResult<NoiseModel> = noise_keys
            .filter(id.eq(room_id))
            .first(conn);

        match result {
            Ok(entity) => {
                log::info!("Got ECDSA keypair for room {}", room_id);
                Self::from_entity(entity)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn delete_key(&self, room_id: &str) -> Result<()> {
        log::info!("Deleting ECDSA keypair for room {}", room_id);
        let mut conn = self.db_pool.get()?;
        diesel::delete(noise_keys.filter(id.eq(room_id))).execute(&mut conn)?;

        log::info!("Deleted ECDSA keypair for room {}", room_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use diesel::r2d2::ConnectionManager;
    use diesel::r2d2::Pool;

    use crate::services::connection::establish_connection;

    use super::*;

    fn setup_test_db() -> Pool<ConnectionManager<SqliteConnection>> {
        let database_url = ":memory:"; // SQLite in-memory database
        establish_connection(Some(database_url.to_string()))
    }

    #[test]
    fn test_generate_ecdsa_keypair() {
        let room_id = "test_room";
        let keypair = NoiseKeyService::generate_ecdsa_keypair(room_id);
        assert!(keypair.is_ok());
    }

    #[test]
    fn test_create_and_get_key() {
        let db_pool = setup_test_db();
        let service = NoiseKeyService::new(db_pool);
        let room_id = "test_room_1";

        let create_result = service.create_key(room_id);
        assert!(create_result.is_ok());

        let get_key_result = service.get_key(room_id);
        assert!(get_key_result.is_ok());
    }

    #[test]
    fn test_delete_key() {
        let db_pool = setup_test_db();
        let service = NoiseKeyService::new(db_pool);
        let room_id = "test_room_2";

        let create_result = service.create_key(room_id);
        assert!(create_result.is_ok());

        let delete_result = service.delete_key(room_id);
        assert!(delete_result.is_ok());

        let get_key_after_delete = service.get_key(room_id);
        assert!(get_key_after_delete.is_err()); // Expect an error after deleting the key
    }
}
