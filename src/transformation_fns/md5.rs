use crate::transformation_fns::TransformationFn;

/// MD5-Hashing
pub static MD5_HASHING: TransformationFn = md5;

fn md5(input: &String) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{:x}", digest)
}
