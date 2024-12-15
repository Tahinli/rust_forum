use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserContact {
    pub user_id: i64,
    pub contact_id: i64,
}
