use std::sync::mpsc;

use crate::models::error::*;

pub enum ControlMessage {
    Stop,
    // Add more control commands if needed.
}

pub struct SwarmController {
    pub(crate) sender: mpsc::Sender<ControlMessage>,
}

impl SwarmController {
    pub fn stop(&self) -> Result<()> {
        self.sender.send(ControlMessage::Stop)?;
        Ok(())
    }
}
