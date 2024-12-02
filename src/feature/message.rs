use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub sender_email: String,
    pub receiver_email: String,
    pub message: String,
    pub execution_time: DateTime<Utc>,
}

impl Message {
    pub async fn new(sender_email: String, receiver_email: String, message: String) -> Self {
        Self {
            sender_email,
            receiver_email,
            message,
            execution_time: Utc::now(),
        }
    }
}
