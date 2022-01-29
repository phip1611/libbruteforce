use sha2::digest::Output;
use sha2::{Digest, Sha256};
use crate::TargetHashAndHashFunction;

/// Returns a [`TargetHashAndHashFunction`] object that does sha256 hashing.
/// It gets initialized with the value we want to crack. The value we want to crack
/// is a hash in string representation.
pub fn sha256_hashing(target_hash_as_str: &str) -> TargetHashAndHashFunction<Sha256Hash> {
    TargetHashAndHashFunction::new(
        target_hash_as_str,
        sha256,
        str_to_sha256_hash
    )
}

pub type Sha256Hash = Output<Sha256>;

fn sha256(input: &str) -> Sha256Hash {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize()
}

fn str_to_sha256_hash(s: &str) -> Sha256Hash {
    let mut target = [0u8; 32];
    hex::decode_to_slice(s, &mut target).expect("Not a sha256 hash");
    target.into()
}

#[cfg(test)]
mod tests {
    use crate::TargetHashAndHashFunctionTrait;
    use super::*;

    #[test]
    fn test_sha256() {
        let input = "sha256";
        let expected_hash = "5d5b09f6dcb2d53a5fffc60c4ac0d55fabdf556069d6631545f42aa6e3500f2e";
        assert!(sha256_hashing(expected_hash).hash_matches(input));
    }
}
