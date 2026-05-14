use std::{env, net::SocketAddr, time::Duration};

#[derive(Debug, Clone)]
pub struct Settings {
    pub addr: SocketAddr,
    pub request_timeout: Duration,
    pub max_body_bytes: usize,
}

impl Settings {
    pub fn from_env() -> anyhow::Result<Self> {
        let addr = env::var("APP_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
            .parse()?;

        let request_timeout_secs = env::var("REQUEST_TIMEOUT_SECS")
            .ok()
            .and_then(|raw| raw.parse::<u64>().ok())
            .unwrap_or(5);

        let max_body_bytes = env::var("MAX_BODY_BYTES")
            .ok()
            .and_then(|raw| raw.parse::<usize>().ok())
            .unwrap_or(64 * 1024);

        Ok(Self {
            addr,
            request_timeout: Duration::from_secs(request_timeout_secs),
            max_body_bytes,
        })
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:3000".parse().expect("valid default address"),
            request_timeout: Duration::from_secs(5),
            max_body_bytes: 64 * 1024,
        }
    }
}
