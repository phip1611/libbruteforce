use sha1::{Digest, Sha1};
use sha1::digest::Output;
use crate::TargetHashAndHashFunction;

/// Returns a [`TargetHashAndHashFunction`] object that does [`sha1`] hashing.
/// It gets initialized with the value we want to crack. The value we want to crack
/// is a hash in string representation.
pub fn sha1_hashing(target_hash_as_str: &str) -> TargetHashAndHashFunction<Sha1Hash> {
    TargetHashAndHashFunction::new(
        target_hash_as_str,
        sha1,
        str_to_sha1_hash
    )
}

pub type Sha1Hash = Output<Sha1>;

fn sha1(input: &str) -> Sha1Hash {
    let mut m = Sha1::default();
    m.update(input);
    m.finalize()
}

fn str_to_sha1_hash(s: &str) -> Sha1Hash {
    let mut target = [0u8; 20];
    hex::decode_to_slice(s, &mut target).expect("Not a sha256 hash");
    target.into()
}

#[cfg(test)]
mod tests {
    use crate::TargetHashAndHashFunctionTrait;
    use super::*;

    #[test]
    fn test_sha1() {
        let input = "sha1";
        let expected_hash = "415ab40ae9b7cc4e66d6769cb2c08106e8293b48";
        assert!(sha1_hashing(expected_hash).hash_matches(input));
    }
}
