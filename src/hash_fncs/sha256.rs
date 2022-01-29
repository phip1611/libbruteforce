/*
MIT License

Copyright (c) 2022 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
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
