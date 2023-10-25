pub struct RoomMember {
  id: String,
  room_id: String,
}

impl RoomMember {
  pub fn new(id: String, room_id: String) -> Self {
    Self { id, room_id }
  }

  pub fn get_id(&self) -> &String {
    &self.id
  }

  pub fn get_room_id(&self) -> &String {
    &self.room_id
  }

}
