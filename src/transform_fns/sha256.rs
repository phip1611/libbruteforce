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

pub fn sha256_hash_to_hex_string(hash: &Sha256Hash) -> String {
    let mut buf = [0; 64];
    hex::encode_to_slice(hash, &mut buf).unwrap();
    String::from_utf8_lossy(&buf).to_string()
}
