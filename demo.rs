use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    service_name: String,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    email: String,
}

#[derive(Debug, Serialize)]
struct CreateUserResponse {
    id: u64,
    email: String,
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<CreateUserResponse>), ApiError> {
    if !req.email.contains('@') {
        return Err(ApiError::BadRequest("email must contain @"));
    }

    tracing::info!(
service = %state.service_name,
"creating user"
);

    Ok((StatusCode::CREATED, Json(CreateUserResponse {
        id: 1,
        email: req.email,
    })))
}
