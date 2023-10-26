pub struct RoomOption {
    pub id: Option<String>,
    pub name: String,
}

impl RoomOption {
    pub fn new(id: Option<&str>, name: &str) -> Self {
        Self {
            id: id.map(|id| id.to_string()),
            name: name.to_string(),
        }
    }
}
