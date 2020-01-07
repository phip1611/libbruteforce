use std::time::Instant;

pub fn seconds_as_fraction(instant: &Instant) -> f64 {
    let millis = instant.elapsed().as_millis() as f64;
    millis / 1000 as f64
}
