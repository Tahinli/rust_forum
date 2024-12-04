use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub creation_time: DateTime<Utc>,
    pub poster_id: i64,
    pub post: String,
}
