/// A unit of background work.
///
/// The idempotency key is intentionally part of the model because production
/// retry systems must avoid duplicate side effects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    pub id: u64,
    pub idempotency_key: String,
    pub payload: String,
}

impl Job {
    pub fn new(id: u64, payload: impl Into<String>) -> Self {
        Self {
            id,
            idempotency_key: format!("job-{id}"),
            payload: payload.into(),
        }
    }

    pub fn with_idempotency_key(
        id: u64,
        idempotency_key: impl Into<String>,
        payload: impl Into<String>,
    ) -> Self {
        Self {
            id,
            idempotency_key: idempotency_key.into(),
            payload: payload.into(),
        }
    }
}
