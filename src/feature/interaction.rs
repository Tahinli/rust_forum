use serde::{Deserialize, Serialize};

use crate::database::interaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub interaction_id: i64,
    pub name: String,
}

impl Interaction {
    pub async fn create(name: &String) -> Result<Interaction, sqlx::Error> {
        interaction::create(name).await
    }

    pub async fn read(interaction_id: &i64) -> Result<Interaction, sqlx::Error> {
        interaction::read(interaction_id).await
    }

    pub async fn update(interaction_id: &i64, name: &String) -> Result<Interaction, sqlx::Error> {
        interaction::update(interaction_id, name).await
    }

    pub async fn delete(interaction_id: &i64) -> Result<Interaction, sqlx::Error> {
        interaction::delete(interaction_id).await
    }

    pub async fn read_all() -> Result<Vec<Interaction>, sqlx::Error> {
        interaction::read_all().await
    }
}
