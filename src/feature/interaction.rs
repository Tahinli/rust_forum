use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub id: i64,
    pub name: String,
}
