use crate::{app::AppState, error::ApiError};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub id: u64,
    pub email: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<CreateUserResponse>), ApiError> {
    let email = req.email.trim().to_lowercase();
    validate_email(&email)?;

    // Avoid logging raw email addresses in production logs.
    tracing::info!(
        service = %state.service_name,
        email_domain = %email_domain(&email).unwrap_or("unknown"),
        "creating user"
    );

    Ok((
        StatusCode::CREATED,
        Json(CreateUserResponse {
            id: state.allocate_user_id(),
            email,
        }),
    ))
}

fn validate_email(email: &str) -> Result<(), ApiError> {
    if email.len() > 254 {
        return Err(ApiError::BadRequest("email is too long"));
    }

    let Some((local, domain)) = email.split_once('@') else {
        return Err(ApiError::BadRequest("email must contain @"));
    };

    if local.is_empty() || domain.is_empty() || !domain.contains('.') {
        return Err(ApiError::BadRequest("email format is invalid"));
    }

    Ok(())
}

fn email_domain(email: &str) -> Option<&str> {
    email.split_once('@').map(|(_, domain)| domain)
}

#[cfg(test)]
mod tests {
    use super::validate_email;

    #[test]
    fn valid_email_is_accepted() {
        assert!(validate_email("ada@example.com").is_ok());
    }

    #[test]
    fn invalid_email_is_rejected() {
        assert!(validate_email("not-an-email").is_err());
    }
}
