use derive_more::From;
use diesel::prelude::*;
use getset::*;
use serde::{Deserialize, Serialize};

use crate::schema::rooms;

#[derive(From, Getters, MutGetters, Setters, Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = rooms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Room {
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub")]
    pub id: String,

    #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_initialization() {
        let id = "test-room-id".to_string();
        let name = "test-room-name".to_string();

        let room = Room::from((id.clone(), name.clone()));

        assert_eq!(room.id().clone(), id);
        assert_eq!(room.name().clone(), name);
    }

    #[test]
    fn test_get_id() {
        let id = "sample-id".to_string();
        let room = Room::from((id.clone(), "sample-name".to_string()));

        assert_eq!(room.name().clone(), id);
    }

    #[test]
    fn test_get_name() {
        let name = "sample-name".to_string();
        let room = Room::from(("sample-id".to_string(), name.clone()));

        assert_eq!(room.name().clone(), name);
    }
}
