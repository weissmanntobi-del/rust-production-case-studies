use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_api_case_study::{
    app::{build_router_with_state, AppState},
    config::Settings,
};
use std::sync::atomic::Ordering;
use tower::ServiceExt;

#[tokio::test]
async fn health_returns_no_content() {
    let app = build_router_with_state(Settings::default(), AppState::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn readyz_returns_no_content_when_ready() {
    let app = build_router_with_state(Settings::default(), AppState::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn readyz_returns_service_unavailable_when_not_ready() {
    let state = AppState::default();
    state.ready.store(false, Ordering::Relaxed);
    let app = build_router_with_state(Settings::default(), state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn create_user_accepts_valid_email() {
    let app = build_router_with_state(Settings::default(), AppState::default());

    let body = serde_json::json!({ "email": "Ada@Example.com" }).to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn create_user_rejects_invalid_email() {
    let app = build_router_with_state(Settings::default(), AppState::default());

    let body = serde_json::json!({ "email": "invalid" }).to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
