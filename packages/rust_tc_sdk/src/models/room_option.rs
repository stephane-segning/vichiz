use derive_more::From;
use getset::*;
use serde::*;

#[derive(From, Getters, MutGetters, Setters, Serialize, Debug, Deserialize)]
pub struct RoomOption {
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
    pub id: Option<String>,

    #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_option_new_with_id() {
        let name = "Test Room";

        let room_option = RoomOption::from((None, name.clone().to_string()));

        assert_eq!(room_option.id, None);
        assert_eq!(room_option.name, name.to_string());
    }

    #[test]
    fn test_room_option_new_without_id() {
        let name = "Test Room";

        let room_option = RoomOption::from((None, name.clone().to_string()));

        assert_eq!(room_option.id, None);
        assert_eq!(room_option.name, name.to_string());
    }
}
