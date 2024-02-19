use tokio::sync::mpsc;

#[derive(PartialEq)]
pub enum ControlMessage {
    Stop,
    // Add more control commands if needed.
}

pub struct SwarmController {
    pub(crate) sender: mpsc::Sender<ControlMessage>,
}

impl SwarmController {
    pub async fn stop(&self) {
        if let Err(e) = self.sender.send(ControlMessage::Stop).await {
            log::error!("Swarm controller error:: {:?}", e);
        }
    }
}
