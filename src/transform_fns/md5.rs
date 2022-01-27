use md5::digest::Output;
use md5::{Digest, Md5};

use crate::transform_fns::TransformFn;

/// MD5-Hashing
pub const MD5_HASHING: TransformFn<Md5Hash> = md5;

pub type Md5Hash = Output<Md5>;

fn md5(input: &str) -> Md5Hash {
    let mut m = Md5::new();
    m.update(input);
    m.finalize()
}

pub fn str_to_md5_hash(s: &str) -> Md5Hash {
    let mut target = [0u8; 16];
    hex::decode_to_slice(s, &mut target).expect("Not a sha256 hash");
    target.into()
}
