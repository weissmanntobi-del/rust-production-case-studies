use crate::{config::Settings, routes};
use axum::http::StatusCode;
use axum::Router;
use http::Method;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

#[derive(Clone)]
pub struct AppState {
    pub service_name: Arc<str>,
    pub ready: Arc<AtomicBool>,
    pub next_user_id: Arc<AtomicU64>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            service_name: Arc::from("axum-api-case-study"),
            ready: Arc::new(AtomicBool::new(true)),
            next_user_id: Arc::new(AtomicU64::new(1)),
        }
    }
}

impl AppState {
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    pub fn allocate_user_id(&self) -> u64 {
        self.next_user_id.fetch_add(1, Ordering::Relaxed)
    }
}

pub fn build_router(settings: Settings) -> Router {
    build_router_with_state(settings, AppState::default())
}

pub fn build_router_with_state(settings: Settings, state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);

    Router::new()
        .merge(routes::router())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            settings.request_timeout,
        ))
        .layer(RequestBodyLimitLayer::new(settings.max_body_bytes))
        .layer(cors)
        .with_state(state)
}
