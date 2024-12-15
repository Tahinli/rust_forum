use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostInteraction {
    pub post_creation_time: DateTime<Utc>,
    pub interaction_id: i64,
    pub user_id: i64,
    pub interaction_time: DateTime<Utc>,
}
