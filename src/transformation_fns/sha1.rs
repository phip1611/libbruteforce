use sha1::Sha1;
use crate::transformation_fns::TransformationFn;

/// Sha1-Hashing
pub static SHA1_HASHING: TransformationFn = sha1;

fn sha1(input: &String) -> String {
    let mut m = Sha1::new();
    m.update(input.as_ref());
    m.digest().to_string()
}
