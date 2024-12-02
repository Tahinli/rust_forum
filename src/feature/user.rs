use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub email: String,
    pub phone: String,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: Vec<String>,
    pub surname: Vec<String>,
    pub gender: bool,
    pub birth_date: NaiveDate,
    pub email: String,
    pub role: Role,
}

impl User {
    pub async fn new(
        name: Vec<String>,
        surname: Vec<String>,
        gender: bool,
        birth_date: NaiveDate,
        email: String,
    ) -> Self {
        Self {
            name,
            surname,
            gender,
            birth_date,
            email,
            role: Role::User,
        }
    }
}
