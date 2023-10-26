use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::services::schema::noise_keys;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = noise_keys)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_model_initialization() {
        let id = "sample-id";
        let private = vec![1, 2, 3];
        let public = vec![4, 5, 6];

        let model = NoiseModel::new(id, private.clone(), public.clone());

        assert_eq!(model.id, id);
        assert_eq!(model.private, private);
        assert_eq!(model.public, public);
    }
}
