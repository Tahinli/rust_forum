use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub role_id: i64,
    pub permission_id: i64,
}
