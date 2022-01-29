use crate::{TargetHashAndHashFunction, TargetHashInput};
use sha1::digest::Output;
use sha1::{Digest, Sha1};

/// Returns a [`TargetHashAndHashFunction`] object that does `sha1` hashing using [`mod@sha2`].
/// It gets initialized with a object of type [`TargetHashInput`].
pub fn sha1_hashing(input: TargetHashInput) -> TargetHashAndHashFunction<Sha1Hash> {
    TargetHashAndHashFunction::new(input, sha1, str_to_sha1_hash, sha1_hash_to_string)
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

fn sha1_hash_to_string(hash: &Sha1Hash) -> String {
    hex::encode(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1() {
        let input = "sha1";
        let expected_hash = "415ab40ae9b7cc4e66d6769cb2c08106e8293b48";
        assert!(sha1_hashing(TargetHashInput::HashAsStr(expected_hash)).hash_matches(input));
    }
}
