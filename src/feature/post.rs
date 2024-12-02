use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub poster_email: String,
    pub post: String,
}

impl Post {
    pub async fn new(poster_email: String, post: String) -> Self {
        Self { poster_email, post }
    }
}
