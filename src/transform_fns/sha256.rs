use sha2::{Digest, Sha256};

use crate::transform_fns::TransformFn;

/// SHA256-Hashing
pub static SHA256_HASHING: TransformFn = sha256;

fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(input);
    let result = hasher.result();
    format!("{:x}", result)
}
