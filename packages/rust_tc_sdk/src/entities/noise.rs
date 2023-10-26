use derive_more::From;
use diesel::prelude::*;
use getset::*;
use serde::{Deserialize, Serialize};

use crate::schema::noise_keys;

#[derive(From, Getters, MutGetters, Setters, Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = noise_keys)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NoiseModel {
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    pub id: String,

    #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
    pub private: Vec<u8>,

    #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
    pub public: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_model_initialization() {
        let id = "sample-id";
        let private = vec![1, 2, 3];
        let public = vec![4, 5, 6];

        let model = NoiseModel::from((id.to_string(), private.clone(), public.clone()));

        assert_eq!(model.id, id);
        assert_eq!(model.private, private);
        assert_eq!(model.public, public);
    }
}
