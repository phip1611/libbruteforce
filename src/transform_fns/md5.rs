use crate::transform_fns::TransformFn;

/// MD5-Hashing
pub static MD5_HASHING: TransformFn = md5;

fn md5(input: &String) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{:x}", digest)
}
