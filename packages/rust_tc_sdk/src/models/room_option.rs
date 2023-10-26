pub struct RoomOption {
  pub id: Option<String>,
  pub name: String,
}

impl RoomOption {
  pub fn new(id: Option<String>, name: String) -> Self {
    Self { id, name }
  }
}
