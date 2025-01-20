use serde::{Deserialize, Serialize};

use crate::database::interaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub id: i64,
    pub name: String,
}

impl Interaction {
    pub async fn create(name: &String) -> Result<Interaction, sqlx::Error> {
        interaction::create(name).await
    }

    pub async fn read(id: &i64) -> Result<Interaction, sqlx::Error> {
        interaction::read(id).await
    }

    pub async fn update(id: &i64, name: &String) -> Result<Interaction, sqlx::Error> {
        interaction::update(id, name).await
    }

    pub async fn delete(id: &i64) -> Result<Interaction, sqlx::Error> {
        interaction::delete(id).await
    }

    pub async fn read_all() -> Result<Vec<Interaction>, sqlx::Error> {
        interaction::read_all().await
    }
}
