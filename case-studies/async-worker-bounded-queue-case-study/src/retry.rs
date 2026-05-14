use std::time::Duration;

/// Exponential backoff with deterministic jitter.
///
/// The jitter is deterministic to keep tests predictable. In a real distributed
/// system, random jitter is also acceptable.
pub fn backoff_delay(base: Duration, max: Duration, attempt: usize, job_id: u64) -> Duration {
    let attempt = attempt.max(1);
    let exponent = (attempt - 1).min(10) as u32;
    let multiplier = 1u128 << exponent;

    let base_ms = base.as_millis().max(1);
    let max_ms = max.as_millis().max(base_ms);
    let jitter_ms = ((job_id % 17) + 1) as u128 * 7;
    let total_ms = (base_ms * multiplier + jitter_ms).min(max_ms);

    Duration::from_millis(total_ms as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_is_capped() {
        let delay = backoff_delay(
            Duration::from_millis(100),
            Duration::from_millis(250),
            10,
            42,
        );
        assert_eq!(delay, Duration::from_millis(250));
    }
}
