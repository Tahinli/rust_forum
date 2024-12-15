use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::role_permission;

#[derive(Debug, Serialize, Deserialize)]
pub struct RolePermission {
    pub role_id: i64,
    pub permission_id: i64,
}

impl RolePermission {
    pub async fn create(
        role_id: &i64,
        permission_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<RolePermission, sqlx::Error> {
        role_permission::create(role_id, permission_id, database_connection).await
    }

    pub async fn read(
        role_id: &i64,
        permission_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<RolePermission, sqlx::Error> {
        role_permission::read(role_id, permission_id, database_connection).await
    }

    pub async fn update(
        role_id: &i64,
        permission_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<RolePermission, sqlx::Error> {
        role_permission::update(role_id, permission_id, database_connection).await
    }
    pub async fn delete(
        role_id: &i64,
        permission_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<RolePermission, sqlx::Error> {
        role_permission::delete(role_id, permission_id, database_connection).await
    }

    pub async fn read_all_for_role(
        role_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<RolePermission>, sqlx::Error> {
        role_permission::read_all_for_role(role_id, database_connection).await
    }

    pub async fn delete_all_for_role(
        role_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<RolePermission>, sqlx::Error> {
        role_permission::delete_all_for_role(role_id, database_connection).await
    }
}
