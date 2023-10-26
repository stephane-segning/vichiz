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
  pub fn new(id: String, name: String) -> Self {
    Self { id, name }
  }

  pub fn get_id(&self) -> &String {
    &self.id
  }

  pub fn get_name(&self) -> &String {
    &self.name
  }
}

