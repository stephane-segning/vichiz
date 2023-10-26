use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = crate::services::schema::rooms)]
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

