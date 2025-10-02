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
use crate::CrackTarget;
use std::fmt::{Debug, Formatter};

/// Helper type to create instances of [`TargetHashAndHashFunction`].
#[derive(Debug)]
pub enum TargetHashInput<'a> {
    /// The provided input is already a valid hash but as (hex) string representation.
    HashAsStr(&'a str),
    /// The provided input is plain text and needs to be hashed by the constructor.
    /// This is useful for tests, examples, and debugging. For real applications you
    /// may want to use [`Self::HashAsStr`].
    Plaintext(&'a str),
}

/// Abstraction over a hashing algorithm and the target hash that needs to be cracked.
///
/// `T` is of type [`CrackTarget`]. This generic struct exists so that hashes of type
/// [`CrackTarget`] can be checked independent of the hashing algorithm. This is
/// more efficient than transforming every hash to a string and compare the hash
/// string representations afterwards.
pub struct TargetHashAndHashFunction<T: CrackTarget> {
    /// The target hash we want to crack.
    target_hash: T,
    /// Function that calculates the hash of type `T` of the given input plain text.
    hash_fn: fn(input: &str) -> T,
    /// Function that transforms a `T` in string representation to a real `T`.
    /// For example, this transforms a `sha256` string representation to the runtime
    /// type the hashing library uses.
    hash_str_repr_to_hash_type_fn: fn(hash_as_string: &str) -> T,
    /// Function that transform the hash type to a string representation. Usually, this
    /// will return a hex string that represents the hash.
    hash_type_to_str_repr_fn: fn(hash: &T) -> String,
}

impl<T: CrackTarget> TargetHashAndHashFunction<T> {
    /// Constructor that takes a hashing function and a target hash.
    ///
    /// # Parameters
    /// * `target_hash` String representation of the target hash we want to crack.
    ///                 This is usually the hex string representation of a sha256 hash or so.
    /// * `hash_fn` Transforms a plain input password/guess of type `str` to the target hash.
    ///                  This is the hashing function.
    /// * `hash_str_repr_to_hash_type_fn` Function that can take the argument `target_hash`
    ///                                   and transform it to the target hashing type. This
    ///                                   usually transforms the hex string that represents the
    ///                                   hash to bytes in memory.
    /// * `hash_type_to_str_repr_fn` Function that transform the hash type to a string representation.
    ///                             Usually, this will return a hex string that represents the hash.
    pub fn new(
        target_hash: TargetHashInput,
        hash_fn: fn(&str) -> T,
        hash_str_repr_to_hash_type_fn: fn(hash_as_string: &str) -> T,
        hash_type_to_str_repr_fn: fn(hash: &T) -> String,
    ) -> Self {
        let target_hash = match target_hash {
            TargetHashInput::HashAsStr(hash_str) => hash_str_repr_to_hash_type_fn(hash_str),
            TargetHashInput::Plaintext(input) => hash_fn(input),
        };
        Self {
            target_hash,
            hash_fn,
            hash_str_repr_to_hash_type_fn,
            hash_type_to_str_repr_fn,
        }
    }
}

impl<T: CrackTarget> Debug for TargetHashAndHashFunction<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TargetHashAndHashFunction")
            .field("target_hash", &self.target_hash)
            .field("hash_fn", &"<func impl>")
            .field("target_str_repr_to_hash_type_fn", &"<func impl>")
            .field("hash_type_to_str_repr_fn", &"<func impl>")
            .finish()
    }
}

impl<T: CrackTarget> TargetHashAndHashFunction<T> {
    /// Transforms the (hex) string representation into the type
    /// the hash implementation uses to represent hashes.
    pub fn hash_str_repr_to_hash_type(&self, hash_as_string: &str) -> T {
        (self.hash_str_repr_to_hash_type_fn)(hash_as_string)
    }

    /// Hashes a value.
    pub fn hash(&self, input: &str) -> T {
        (self.hash_fn)(input)
    }

    /// Returns the target hash that we want to crack.
    pub const fn target_hash(&self) -> &T {
        &self.target_hash
    }

    /// Returns a (hex) string representation of the hash.
    pub fn hash_type_to_str_repr(&self, hash: &T) -> String {
        (self.hash_type_to_str_repr_fn)(hash)
    }

    /// Hashes the input value and returns if it equals the target hash.
    /// If so, the hash got cracked.
    pub fn hash_matches(&self, input: &str) -> bool {
        (self.hash_fn)(input) == self.target_hash
    }
}
