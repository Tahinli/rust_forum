use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::database::routing_permission;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoutingPermission {
    pub routing_id: i64,
    pub permission_id: i64,
}

impl RoutingPermission {
    pub async fn create(
        routing_id: &i64,
        permission_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<RoutingPermission, sqlx::Error> {
        routing_permission::create(routing_id, permission_id, database_connection).await
    }

    pub async fn read(
        routing_id: &i64,
        permission_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<RoutingPermission, sqlx::Error> {
        routing_permission::read(routing_id, permission_id, database_connection).await
    }

    pub async fn update(
        routing_id: &i64,
        permission_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<RoutingPermission, sqlx::Error> {
        routing_permission::update(routing_id, permission_id, database_connection).await
    }
    pub async fn delete(
        routing_id: &i64,
        permission_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<RoutingPermission, sqlx::Error> {
        routing_permission::delete(routing_id, permission_id, database_connection).await
    }

    pub async fn read_all_for_routing(
        routing_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<RoutingPermission>, sqlx::Error> {
        routing_permission::read_all_for_routing(routing_id, database_connection).await
    }

    pub async fn delete_all_for_routing(
        routing_id: &i64,
        database_connection: &Pool<Postgres>,
    ) -> Result<Vec<RoutingPermission>, sqlx::Error> {
        routing_permission::delete_all_for_routing(routing_id, database_connection).await
    }
}
