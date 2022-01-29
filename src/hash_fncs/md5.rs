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
//! Exports the hashing algorithm [`md5_hashing`].

use crate::{TargetHashAndHashFunction, TargetHashInput};
use md5::digest::Output;
use md5::{Digest, Md5};

/// Returns a [`TargetHashAndHashFunction`] object that does [`mod@md5`] hashing.
/// It gets initialized with a object of type [`TargetHashInput`].
pub fn md5_hashing(input: TargetHashInput) -> TargetHashAndHashFunction<Md5Hash> {
    TargetHashAndHashFunction::new(input, md5, str_to_md5_hash, md5_hash_to_string)
}

pub(crate) type Md5Hash = Output<Md5>;

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

fn md5_hash_to_string(hash: &Md5Hash) -> String {
    hex::encode(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let input = "md5";
        let expected_hash = "1bc29b36f623ba82aaf6724fd3b16718";
        assert!(md5_hashing(TargetHashInput::HashAsStr(expected_hash)).hash_matches(input));
    }
}
