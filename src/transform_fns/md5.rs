use md5::digest::Output;
use md5::{Digest, Md5};
use crate::TargetHashAndHashFunction;

/// Returns a [`TargetHashAndHashFunction`] object that does [`md5`] hashing.
/// It gets initialized with the value we want to crack. The value we want to crack
/// is a hash in string representation.
pub fn md5_hashing(target_hash_as_str: &str) -> TargetHashAndHashFunction<Md5Hash> {
    TargetHashAndHashFunction::new(
        target_hash_as_str,
        md5,
        str_to_md5_hash
    )
}

pub type Md5Hash = Output<Md5>;

fn md5(input: &str) -> Md5Hash {
    let mut m = Md5::new();
    m.update(input);
    m.finalize()
}

fn str_to_md5_hash(s: &str) -> Md5Hash {
    let mut target = [0u8; 16];
    hex::decode_to_slice(s, &mut target).expect("Not a md5 hash");
    target.into()
}

#[cfg(test)]
mod tests {
    use crate::TargetHashAndHashFunctionTrait;
    use super::*;

    #[test]
    fn test_md5() {
        let input = "md5";
        let expected_hash = "1bc29b36f623ba82aaf6724fd3b16718";
        assert!(md5_hashing(expected_hash).hash_matches(input));
    }
}
