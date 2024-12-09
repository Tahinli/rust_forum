use std::fmt::Display;

use sqlx::{Pool, Postgres};

use crate::feature::permission::Permission;

#[derive(Debug)]
pub enum PermissionTable {
    Role,
    User,
    Post,
    Comment,
    Interaction,
    PostInteraction,
    CommentInteraction,
}

impl Display for PermissionTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default_identifier = "permission_";
        match self {
            PermissionTable::Role => write!(f, "{}{}", default_identifier, "role"),
            PermissionTable::User => write!(f, "{}{}", default_identifier, "user"),
            PermissionTable::Post => write!(f, "{}{}", default_identifier, "post"),
            PermissionTable::Comment => write!(f, "{}{}", default_identifier, "comment"),
            PermissionTable::Interaction => write!(f, "{}{}", default_identifier, "interaction"),
            PermissionTable::PostInteraction => {
                write!(f, "{}{}", default_identifier, "post_interaction")
            }
            PermissionTable::CommentInteraction => {
                write!(f, "{}{}", default_identifier, "comment_interaction")
            }
        }
    }
}

pub async fn create(
    role_id: &i64,
    create_self: &bool,
    read_self: &bool,
    update_self: &bool,
    delete_self: &bool,
    create_other: &bool,
    read_other: &bool,
    update_other: &bool,
    delete_other: &bool,
    create_lower: &bool,
    read_lower: &bool,
    update_lower: &bool,
    delete_lower: &bool,
    permission_table: &PermissionTable,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
            INSERT INTO "permission_role"(role_id, create_self, read_self, update_self, delete_self, create_other, read_other, update_other, delete_other, create_lower, read_lower, update_lower, delete_lower) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) 
            RETURNING *
        "#,
        role_id,
        create_self,
        read_self,
        update_self,
        delete_self,
        create_other,
        read_other,
        update_other,
        delete_other,
        create_lower,
        read_lower,
        update_lower,
        delete_lower,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn read(
    role_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
            SELECT * FROM "permission_role" WHERE "role_id" = $1
        "#,
        role_id
    )
    .fetch_one(database_connection)
    .await
}

pub async fn update(
    role_id: &i64,
    create_self: &bool,
    read_self: &bool,
    update_self: &bool,
    delete_self: &bool,
    create_other: &bool,
    read_other: &bool,
    update_other: &bool,
    delete_other: &bool,
    create_lower: &bool,
    read_lower: &bool,
    update_lower: &bool,
    delete_lower: &bool,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
        UPDATE "permission_role" SET "create_self" = $2, "read_self" = $3, "update_self" = $4, "delete_self" = $5, "create_other" = $6, "read_other" = $7, "update_other" = $8, "delete_other" = $9, "create_lower" = $10, "read_lower" = $11, "update_lower" = $12, "delete_lower" = $13 WHERE "role_id" = $1
        RETURNING *
    "#,
        role_id,
        create_self,
        read_self,
        update_self,
        delete_self,
        create_other,
        read_other,
        update_other,
        delete_other,
        create_lower,
        read_lower,
        update_lower,
        delete_lower,
    )
    .fetch_one(database_connection)
    .await
}

pub async fn delete(
    role_id: &i64,
    database_connection: &Pool<Postgres>,
) -> Result<Permission, sqlx::Error> {
    sqlx::query_as!(
        Permission,
        r#"
        DELETE FROM "permission_role" where "role_id" = $1
        RETURNING *
    "#,
        role_id
    )
    .fetch_one(database_connection)
    .await
}
