use std::collections::HashMap;
use tokio::sync::mpsc;

type ClientId = usize;

#[derive(Debug)]
pub enum HubMessage {
    Register { id: ClientId, tx: mpsc::Sender<String> },
    Unregister { id: ClientId },
    Broadcast { msg: String },
}

pub struct Hub {
    clients: HashMap<ClientId, mpsc::Sender<String>>,
    rx: mpsc::Receiver<HubMessage>,
}

impl Hub {
    // One task owns the map -> all mutation is serialized here (no locks needed).
    pub async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                HubMessage::Register { id, tx } => {
                    self.clients.insert(id, tx);
                }
                HubMessage::Unregister { id } => {
                    self.clients.remove(&id); // drop stops the sender
                }
                HubMessage::Broadcast { msg } => {
                    let mut disconnected = Vec::new();
                    for (id, tx) in &self.clients {
                        // buffer full -> slow client; drop it (sec 12)
                        if let Err(mpsc::error::TrySendError::Full(_)) = tx.try_send(msg.clone()) {
                            disconnected.push(*id);
                        } else if let Err(mpsc::error::TrySendError::Closed(_)) = tx.try_send(msg.clone()) {
                            disconnected.push(*id);
                        }
                    }
                    for id in disconnected {
                        self.clients.remove(&id);
                    }
                }
            }
        }
    }
}
