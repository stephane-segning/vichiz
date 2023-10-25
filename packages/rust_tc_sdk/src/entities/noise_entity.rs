use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = crate::services::schema::noise_keys)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NoiseKey {
  pub id: String,
  pub private: Vec<u8>,
  pub public: Vec<u8>,
}
