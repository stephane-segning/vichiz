use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = crate::services::schema::noise_keys)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NoiseModel {
    pub id: String,
    pub private: Vec<u8>,
    pub public: Vec<u8>,
}

impl NoiseModel {
    pub fn new(id: &str, private: Vec<u8>, public: Vec<u8>) -> Self {
        Self {
            id: id.to_string(),
            private,
            public,
        }
    }
}