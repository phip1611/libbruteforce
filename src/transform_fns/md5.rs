use md5::{
    digest::generic_array::GenericArray,
    Digest,
    Md5,
};

use crate::transform_fns::TransformFn;

/// MD5-Hashing
pub static MD5_HASHING: TransformFn<Md5Hash> = md5;

pub type Md5Hash = GenericArray<u8, <md5::Md5 as Digest>::OutputSize>;

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
