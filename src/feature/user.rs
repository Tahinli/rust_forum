use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub email: String,
    pub phone: String,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub surname: String,
    pub gender: bool,
    pub birth_date: NaiveDate,
    pub email: String,
    pub role_id: i64,
}
