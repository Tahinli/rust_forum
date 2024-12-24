use sqlx::{Pool, Postgres};

use crate::feature::routing_permission::RoutingPermission;

pub async fn create(
    routing_id: &i64,
    permission_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<RoutingPermission, sqlx::Error> {
    sqlx::query_as!(
        RoutingPermission,
        r#"
            INSERT INTO "routing_permission"(routing_id, permission_id) 
            VALUES ($1, $2) 
            RETURNING *
        "#,
        routing_id,
        permission_id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    routing_id: &i64,
    permission_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<RoutingPermission, sqlx::Error> {
    sqlx::query_as!(
        RoutingPermission,
        r#"
            SELECT * FROM "routing_permission" WHERE "routing_id" = $1 AND "permission_id" = $2
        "#,
        routing_id,
        permission_id,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    routing_id: &i64,
    permission_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<RoutingPermission, sqlx::Error> {
    sqlx::query_as!(
        RoutingPermission,
        r#"
        UPDATE "routing_permission" SET "permission_id" = $2 WHERE "routing_id" = $1
        RETURNING *
    "#,
        routing_id,
        permission_id,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    routing_id: &i64,
    permission_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<RoutingPermission, sqlx::Error> {
    sqlx::query_as!(
        RoutingPermission,
        r#"
        DELETE FROM "routing_permission" WHERE "routing_id" = $1 AND "permission_id" = $2
        RETURNING *
    "#,
        routing_id,
        permission_id,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read_all_for_routing(
    routing_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<RoutingPermission>, sqlx::Error> {
    sqlx::query_as!(
        RoutingPermission,
        r#"
            SELECT * FROM "routing_permission" WHERE "routing_id" = $1
        "#,
        routing_id
    )
    .fetch_all(database_connection)
    .await
}

pub async fn delete_all_for_routing(
    routing_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Vec<RoutingPermission>, sqlx::Error> {
    sqlx::query_as!(
        RoutingPermission,
        r#"
        DELETE FROM "routing_permission" WHERE "routing_id" = $1
        RETURNING *
    "#,
        routing_id,
    )
    .fetch_all(database_connection)
    .await
}
