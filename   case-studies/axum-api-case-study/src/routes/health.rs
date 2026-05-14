use crate::{app::AppState, error::ApiError};
use axum::{extract::State, http::StatusCode};

pub async fn health() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn readyz(State(state): State<AppState>) -> Result<StatusCode, ApiError> {
    if state.is_ready() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotReady)
    }
}
