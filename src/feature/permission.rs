use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub role_id: i64,
    pub create_self: bool,
    pub read_self: bool,
    pub update_self: bool,
    pub delete_self: bool,
    pub create_other: bool,
    pub read_other: bool,
    pub update_other: bool,
    pub delete_other: bool,
    pub create_lower: bool,
    pub read_lower: bool,
    pub update_lower: bool,
    pub delete_lower: bool,
}
