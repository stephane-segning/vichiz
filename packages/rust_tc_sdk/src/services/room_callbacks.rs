pub trait RoomCallbacks {
  fn on_new_participant(&mut self, callback: Box<dyn Fn() + Send>);
  fn on_participant_connect(&mut self, callback: Box<dyn Fn() + Send>);
  fn on_participant_disconnect(&mut self, callback: Box<dyn Fn() + Send>);
  fn on_message(&mut self, protocol: &str, callback: Box<dyn Fn(&str) + Send>);
}
