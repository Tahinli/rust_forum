use chrono::{DateTime, Utc};

pub struct Comment {
    pub post_creation_time: DateTime<Utc>,
    pub creation_time: DateTime<Utc>,
    pub commenter_id: i64,
    pub comment: String,
}
