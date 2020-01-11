use sha2::{Sha256, Digest};
use crate::transformation_fns::TransformationFn;

/// SHA256-Hashing
pub static SHA256_HASHING: TransformationFn = sha256;

fn sha256(input: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.input(input);
    let result = hasher.result();
    format!("{:x}", result)
}
