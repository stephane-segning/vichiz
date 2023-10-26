use getset::Getters;
use serde::*;

#[derive(Getters, Serialize, Debug, Deserialize)]
pub struct RoomOption {
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
    pub id: Option<String>,

    #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
    pub name: String,
}

impl RoomOption {
    pub fn new<S: Into<String>>(id: Option<S>, name: S) -> Self {
        Self {
            id: id.map(|s| s.into()),
            name: name.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_option_new_with_id() {
        let name = "Test Room";

        let room_option = RoomOption::new(None, name);

        assert_eq!(room_option.id, None);
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
