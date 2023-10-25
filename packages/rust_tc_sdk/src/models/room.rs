use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
  id: String,
  name: String,
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

pub struct RoomOption {
  pub id: Option<String>,
  pub name: String,
}

impl RoomOption {
  pub fn new(id: Option<String>, name: String) -> Self {
    Self { id, name }
  }
}
