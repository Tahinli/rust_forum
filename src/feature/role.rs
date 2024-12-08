use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
}
