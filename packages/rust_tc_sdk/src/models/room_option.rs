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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_option_new_with_id() {
        let id = Some("123");
        let name = "Test Room";

        let room_option = RoomOption::new(id, name);

        assert_eq!(room_option.id, id.map(|s| s.to_string()));
        assert_eq!(room_option.name, name);
    }

    #[test]
    fn test_room_option_new_without_id() {
        let name = "Test Room";

        let room_option = RoomOption::new(None, name);

        assert_eq!(room_option.id, None);
        assert_eq!(room_option.name, name);
    }
}
