use std::sync::Arc;
use tokio::sync::Mutex;
use crate::types::{Message, Group};

#[derive(Clone)]
pub struct MessageService {
    pending: Arc<Mutex<Vec<Message>>>,
}

impl MessageService {
    pub fn new() -> Self {
        Self {
            pending: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn send_message(&self, msg: Message) {
        let mut pending = self.pending.lock().await;
        pending.push(msg);
    }

    pub async fn create_group(&self, size: usize) -> Option<Group> {
        let mut pending = self.pending.lock().await;

        if pending.is_empty() {
            return None;
        }

        let take = size.min(pending.len());
        let messages = pending.drain(0..take).collect();

        Some(Group { messages })
    }

    pub async fn pending_messages_count(&self) -> usize {
        let pending = self.pending.lock().await;
        pending.len()
    }
}
