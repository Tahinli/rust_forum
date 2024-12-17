use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{feature::user::User, AppState};

#[derive(Debug, Serialize, Deserialize)]
struct CreateUser {
    name: String,
    surname: String,
    gender: bool,
    birth_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateUser {
    id: i64,
    name: String,
    surname: String,
    gender: bool,
    birth_date: NaiveDate,
    role_id: i64,
}

pub fn route(State(app_state): State<AppState>) -> Router<AppState> {
    Router::new()
        .route("/", post(create))
        .route("/:id", get(read))
        .route("/", patch(update))
        .route("/:id", delete(delete_))
        .route("/", get(read_all))
        .route("/names/:name", get(read_all_for_name))
        .route("/surnames/:surname", get(read_all_for_surname))
        .route("/birth_dates/:birth_date", get(read_all_for_birth_date))
        .route("/roles/:role", get(read_all_for_role))
        .route("/genders/:gender", get(read_all_for_gender))
        .route("/ids", get(read_all_id))
        .route("/ids/names/:name", get(read_all_id_for_name))
        .route("/ids/surnames/:surname", get(read_all_id_for_surname))
        .route(
            "/ids/birth_dates/:birth_date",
            get(read_all_id_for_birth_date),
        )
        .route("/ids/roles/:role", get(read_all_id_for_role))
        .route("/ids/genders/:gender", get(read_all_id_for_gender))
        .route("/count", get(count_all))
        .route("/count/names/:name", get(count_all_for_name))
        .route("/count/surnames/:surname", get(count_all_for_surname))
        .route(
            "/count/birth_dates/:birth_date",
            get(count_all_for_birth_date),
        )
        .route("/count/roles/:role", get(count_all_for_role))
        .route("/count/genders/:gender", get(count_all_for_gender))
        .with_state(app_state)
}

async fn create(
    State(app_state): State<AppState>,
    Json(create_user): Json<CreateUser>,
) -> impl IntoResponse {
    match User::create(
        &create_user.name,
        &create_user.surname,
        &create_user.gender,
        &create_user.birth_date,
        &app_state.database_connection,
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

async fn read(State(app_state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match User::read(&id, &app_state.database_connection).await {
        Ok(user) => (StatusCode::OK, Json(serde_json::json!(user))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn update(
    State(app_state): State<AppState>,
    Json(update_user): Json<UpdateUser>,
) -> impl IntoResponse {
    match User::update(
        &update_user.id,
        &update_user.name,
        &update_user.surname,
        &update_user.gender,
        &update_user.birth_date,
        &update_user.role_id,
        &app_state.database_connection,
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

async fn delete_(State(app_state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match User::delete(&id, &app_state.database_connection).await {
        Ok(user) => (StatusCode::NO_CONTENT, Json(serde_json::json!(user))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all(State(app_state): State<AppState>) -> impl IntoResponse {
    match User::read_all(&app_state.database_connection).await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_name(
    State(app_state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match User::read_all_for_name(&name, &app_state.database_connection).await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_surname(
    State(app_state): State<AppState>,
    Path(surname): Path<String>,
) -> impl IntoResponse {
    match User::read_all_for_surname(&surname, &app_state.database_connection).await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_birth_date(
    State(app_state): State<AppState>,
    Path(birth_date): Path<NaiveDate>,
) -> impl IntoResponse {
    match User::read_all_for_birth_date(&birth_date, &app_state.database_connection).await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_role(
    State(app_state): State<AppState>,
    Path(role_id): Path<i64>,
) -> impl IntoResponse {
    match User::read_all_for_role(&role_id, &app_state.database_connection).await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_for_gender(
    State(app_state): State<AppState>,
    Path(gender): Path<bool>,
) -> impl IntoResponse {
    match User::read_all_for_gender(&gender, &app_state.database_connection).await {
        Ok(users) => (StatusCode::OK, Json(serde_json::json!(users))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id(State(app_state): State<AppState>) -> impl IntoResponse {
    match User::read_all_id(&app_state.database_connection).await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id_for_name(
    State(app_state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match User::read_all_id_for_name(&name, &app_state.database_connection).await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id_for_surname(
    State(app_state): State<AppState>,
    Path(surname): Path<String>,
) -> impl IntoResponse {
    match User::read_all_id_for_surname(&surname, &app_state.database_connection).await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id_for_birth_date(
    State(app_state): State<AppState>,
    Path(birth_date): Path<NaiveDate>,
) -> impl IntoResponse {
    match User::read_all_id_for_birth_date(&birth_date, &app_state.database_connection).await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id_for_role(
    State(app_state): State<AppState>,
    Path(role_id): Path<i64>,
) -> impl IntoResponse {
    match User::read_all_id_for_role(&role_id, &app_state.database_connection).await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn read_all_id_for_gender(
    State(app_state): State<AppState>,
    Path(gender): Path<bool>,
) -> impl IntoResponse {
    match User::read_all_id_for_gender(&gender, &app_state.database_connection).await {
        Ok(user_ids) => (StatusCode::OK, Json(serde_json::json!(user_ids))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all(State(app_state): State<AppState>) -> impl IntoResponse {
    match User::count_all(&app_state.database_connection).await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_name(
    State(app_state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match User::count_all_for_name(&name, &app_state.database_connection).await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_surname(
    State(app_state): State<AppState>,
    Path(surname): Path<String>,
) -> impl IntoResponse {
    match User::count_all_for_surname(&surname, &app_state.database_connection).await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_birth_date(
    State(app_state): State<AppState>,
    Path(birth_date): Path<NaiveDate>,
) -> impl IntoResponse {
    match User::count_all_for_birth_date(&birth_date, &app_state.database_connection).await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_role(
    State(app_state): State<AppState>,
    Path(role_id): Path<i64>,
) -> impl IntoResponse {
    match User::count_all_for_role(&role_id, &app_state.database_connection).await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}

async fn count_all_for_gender(
    State(app_state): State<AppState>,
    Path(gender): Path<bool>,
) -> impl IntoResponse {
    match User::count_all_for_gender(&gender, &app_state.database_connection).await {
        Ok(count) => (StatusCode::OK, Json(serde_json::json!(count))),
        Err(err_val) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(err_val.to_string())),
        ),
    }
}
