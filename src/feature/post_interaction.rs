use chrono::{DateTime, Utc};

pub struct PostInteraction {
    pub post_creation_time: DateTime<Utc>,
    pub interaction_id: i64,
    pub interactor_id: i64,
    pub interaction_time: DateTime<Utc>,
}
