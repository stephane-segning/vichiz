use crate::models::error::Error;
use crate::models::message::Message;

pub trait MessageHandler {
  fn send_message(&mut self, room_id: &str, type_str: &str, message: &str) -> Result<(), Error>;
  fn receive_message(&self, room_id: &str, callback: Box<dyn Fn(String, Message) + Send>) -> ();
}
