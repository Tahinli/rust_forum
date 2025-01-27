use std::sync::Arc;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{feature::user::User, routing::middleware::by_uri_then_insert};

#[derive(Debug, Serialize, Deserialize)]
struct CreateUser {
    name: String,
    surname: String,
    gender: bool,
    birth_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateUser {
    name: String,
    surname: String,
    gender: bool,
    birth_date: NaiveDate,
    role_id: i64,
}

pub fn route() -> Router {
    Router::new()
        .route("/", post(create))
        .route(
            "/{user_id}",
            patch(update).route_layer(axum::middleware::from_fn(by_uri_then_insert)),
        )
        .route(
            "/{user_id}",
            delete(delete_).route_layer(axum::middleware::from_fn(by_uri_then_insert)),
        )
        .route("/", get(read_all))
        .route("/names/{name}", get(read_all_for_name))
        .route("/surnames/{surname}", get(read_all_for_surname))
        .route("/birth_dates/{birth_date}", get(read_all_for_birth_date))
        .route("/roles/{role}", get(read_all_for_role))
        .route("/genders/{gender}", get(read_all_for_gender))
        .route("/users_ids", get(read_all_id))
        .route("/users_ids/names/{name}", get(read_all_id_for_name))
        .route(
            "/users_ids/surnames/{surname}",
            get(read_all_id_for_surname),
        )
        .route(
            "/users_ids/birth_dates/{birth_date}",
            get(read_all_id_for_birth_date),
        )
        .route("/users_ids/roles/{role}", get(read_all_id_for_role))
        .route("/users_ids/genders/{gender}", get(read_all_id_for_gender))
        .route("/count", get(count_all))
        .route("/count/names/{name}", get(count_all_for_name))
        .route("/count/surnames/{surname}", get(count_all_for_surname))
        .route(
            "/count/birth_dates/{birth_date}",
            get(count_all_for_birth_date),
        )
        .route("/count/roles/{role}", get(count_all_for_role))
        .route("/count/genders/{gender}", get(count_all_for_gender))
}

async fn create(Json(create_user): Json<CreateUser>) -> impl IntoResponse {
    match User::create(
        &create_user.name,
        &create_user.surname,
        &create_user.gender,
        &create_user.birth_date,
    )
    .await
    {
        Ok(user) => (StatusCode::CREATED, Json(serde_json::json!(user))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    Extension(target_user): Extension<Arc<User>>,
    Json(update_user): Json<UpdateUser>,
) -> impl IntoResponse {
    if update_user.role_id == 0 {
        (StatusCode::FORBIDDEN, Json(serde_json::json!({})))
    } else {
        match User::update(
            &target_user.user_id,
            &update_user.name,
            &update_user.surname,
            &update_user.gender,
            &update_user.birth_date,
            &update_user.role_id,
        )
        .await
        {
            Ok(user) => (StatusCode::ACCEPTED, Json(serde_json::json!(user))),
            Err(err_val) => (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!(err_val.to_string())),
            ),
        }
    }
}

async fn delete_(Extension(target_user): Extension<Arc<User>>) -> impl IntoResponse {
    match User::delete(&target_user.user_id).await {
        Ok(user) => (StatusCode::NO_CONTENT, Json(serde_json::json!(user))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all() -> impl IntoResponse {
    match User::read_all().await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_name(Path(name): Path<String>) -> impl IntoResponse {
    match User::read_all_for_name(&name).await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_surname(Path(surname): Path<String>) -> impl IntoResponse {
    match User::read_all_for_surname(&surname).await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_birth_date(Path(birth_date): Path<NaiveDate>) -> impl IntoResponse {
    match User::read_all_for_birth_date(&birth_date).await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_role(Path(role_id): Path<i64>) -> impl IntoResponse {
    match User::read_all_for_role(&role_id).await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_gender(Path(gender): Path<bool>) -> impl IntoResponse {
    match User::read_all_for_gender(&gender).await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id() -> impl IntoResponse {
    match User::read_all_id().await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id_for_name(Path(name): Path<String>) -> impl IntoResponse {
    match User::read_all_id_for_name(&name).await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id_for_surname(Path(surname): Path<String>) -> impl IntoResponse {
    match User::read_all_id_for_surname(&surname).await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id_for_birth_date(Path(birth_date): Path<NaiveDate>) -> impl IntoResponse {
    match User::read_all_id_for_birth_date(&birth_date).await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id_for_role(Path(role_id): Path<i64>) -> impl IntoResponse {
    match User::read_all_id_for_role(&role_id).await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id_for_gender(Path(gender): Path<bool>) -> impl IntoResponse {
    match User::read_all_id_for_gender(&gender).await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all() -> impl IntoResponse {
    match User::count_all().await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_name(Path(name): Path<String>) -> impl IntoResponse {
    match User::count_all_for_name(&name).await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_surname(Path(surname): Path<String>) -> impl IntoResponse {
    match User::count_all_for_surname(&surname).await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_birth_date(Path(birth_date): Path<NaiveDate>) -> impl IntoResponse {
    match User::count_all_for_birth_date(&birth_date).await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_role(Path(role_id): Path<i64>) -> impl IntoResponse {
    match User::count_all_for_role(&role_id).await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_gender(Path(gender): Path<bool>) -> impl IntoResponse {
    match User::count_all_for_gender(&gender).await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
