use sha2::{digest::generic_array::GenericArray, Digest, Sha256};

use crate::transform_fns::TransformFn;

/// SHA256-Hashing
pub static SHA256_HASHING: TransformFn<Sha256Hash> = sha256;

pub type Sha256Hash = GenericArray<u8, <sha2::Sha256 as Digest>::OutputSize>;

fn sha256(input: &str) -> Sha256Hash {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize()
}

pub fn str_to_sha256_hash(s: &str) -> Sha256Hash {
    let mut target = [0u8; 32];
    hex::decode_to_slice(s, &mut target).expect("Not a sha256 hash");
    target.into()
}
