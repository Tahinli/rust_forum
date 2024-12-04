use chrono::{DateTime, Utc};

pub struct CommentInteraction {
    pub comment_creation_time: DateTime<Utc>,
    pub interaction_id: i64,
    pub interactor_id: i64,
    pub interaction_time: DateTime<Utc>,
}
