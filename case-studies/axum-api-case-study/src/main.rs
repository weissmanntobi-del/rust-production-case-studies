use axum_api_case_study::{app::build_router, config::Settings, telemetry::init_tracing};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let settings = Settings::from_env()?;
    let app = build_router(settings.clone());

    let listener = TcpListener::bind(settings.addr).await?;
    info!(addr = %settings.addr, "server starting");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("server stopped cleanly");
    Ok(())
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sigterm =
            signal(SignalKind::terminate()).expect("failed to install SIGTERM signal handler");

        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("SIGINT received; starting graceful shutdown");
            }
            _ = sigterm.recv() => {
                tracing::info!("SIGTERM received; starting graceful shutdown");
            }
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
        tracing::info!("shutdown signal received; starting graceful shutdown");
    }
}
