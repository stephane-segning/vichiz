use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::services::schema::rooms;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = rooms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Room {
    pub id: String,
    pub name: String,
}

impl Room {
    pub fn new<S: Into<String>>(id: S, name: S) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_initialization() {
        let id = "test-room-id";
        let name = "test-room-name";

        let room = Room::new(id, name);

        assert_eq!(room.get_id(), id);
        assert_eq!(room.get_name(), name);
    }

    #[test]
    fn test_get_id() {
        let id = "sample-id";
        let room = Room::new(id, "sample-name");

        assert_eq!(room.get_id(), id);
    }

    #[test]
    fn test_get_name() {
        let name = "sample-name";
        let room = Room::new("sample-id", name);

        assert_eq!(room.get_name(), name);
    }
}
