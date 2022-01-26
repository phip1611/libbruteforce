use md5::digest::Output;
use sha1::{Digest, Sha1};

use crate::transform_fns::TransformFn;

/// Sha1-Hashing
pub static SHA1_HASHING: TransformFn<Sha1Hash> = sha1;

pub type Sha1Hash = Output<Sha1>;

fn sha1(input: &str) -> Sha1Hash {
    let mut m = Sha1::default();
    m.update(input);
    m.finalize()
}

pub fn str_to_sha1_hash(s: &str) -> Sha1Hash {
    let mut target = [0u8; 20];
    hex::decode_to_slice(s, &mut target).expect("Not a sha256 hash");
    target.into()
}
