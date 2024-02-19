use std::time::{Duration, Instant};

pub fn convert_sec_to_instant(sec: i64) -> Instant {
    Instant::now() + Duration::from_secs(sec as u64)
}