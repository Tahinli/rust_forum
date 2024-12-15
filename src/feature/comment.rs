use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub post_creation_time: DateTime<Utc>,
    pub creation_time: DateTime<Utc>,
    pub user_id: i64,
    pub comment: String,
}
