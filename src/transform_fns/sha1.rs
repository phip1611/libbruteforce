use sha1::Sha1;

use crate::transform_fns::TransformFn;

/// Sha1-Hashing
pub static SHA1_HASHING: TransformFn = sha1;

fn sha1(input: &str) -> String {
    let mut m = Sha1::new();
    m.update(input.as_ref());
    m.digest().to_string()
}
