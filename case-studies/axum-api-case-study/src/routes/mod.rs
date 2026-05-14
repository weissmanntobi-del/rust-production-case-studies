mod health;
mod users;

use crate::app::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/readyz", get(health::readyz))
        .route("/users", post(users::create_user))
}
