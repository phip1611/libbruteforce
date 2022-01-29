use crate::{TargetHashAndHashFunction, TargetHashInput};
use sha2::digest::Output;
use sha2::{Digest, Sha256};

/// Returns a [`TargetHashAndHashFunction`] object that does sha256 hashing using [`mod@sha2`].
/// It gets initialized with a object of type [`TargetHashInput`].
pub fn sha256_hashing(input: TargetHashInput) -> TargetHashAndHashFunction<Sha256Hash> {
    TargetHashAndHashFunction::new(input, sha256, str_to_sha256_hash, sha256_hash_to_string)
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

fn sha256_hash_to_string(hash: &Sha256Hash) -> String {
    hex::encode(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let input = "sha256";
        let expected_hash = "5d5b09f6dcb2d53a5fffc60c4ac0d55fabdf556069d6631545f42aa6e3500f2e";
        assert!(sha256_hashing(TargetHashInput::HashAsStr(expected_hash)).hash_matches(input));
    }
}
