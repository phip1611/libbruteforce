//! This library helps you to brute force hashes (e.g. passwords). It includes a set of pre-configured
//! hashing functions, like md5 or sha256. You can also provide your own hashing function. PLEASE DO NOT
//! use this software to harm someones privacy in any kind! This project was made for fun and for teaching myself
//! new things about Rust.
//!
//! # Minimal example
//! ```ignore
//! use libbruteforce::{BasicCrackParameter, CrackParameter, TargetHashInput};
//! use libbruteforce::hash_fncs::sha256_hashing;
//!
//! // sha256("a+c")
//! let sha256_hash = "3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d";
//!
//! let res = CrackParameter::new(
//!         BasicCrackParameter::new(alphabet, max_len, min_len, false),
//!         sha256_hashing(TargetHashInput::HashAsStr(sha256_hash)),
//! );
//! ```

#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    // clippy::restriction,
    // clippy::pedantic
)]
// now allow a few rules which are denied by the above statement
// --> they are ridiculous and not necessary
#![allow(
    clippy::suboptimal_flops,
    clippy::redundant_pub_crate,
    clippy::fallible_impl_from
)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]

pub use crack::crack;
pub use crack::parameter::{BasicCrackParameter, CrackParameter};
use std::fmt::{Debug, Formatter};

mod crack;
#[cfg(test)]
mod testutil;

// Public API
pub mod symbols;
pub mod hash_fncs;

/// Common trait for crack targets (hashes or plain text to crack). This is the super-type
/// which enables the usage of multiple hashing algorithms. An example that
/// implements this/fulfils the trait requirements is [`String`].
// 'static:
//  - it means the type does not contain any non-static references; i.e. consumes can
//    own implementers of this type easily
//  - https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html
pub trait CrackTarget: 'static + Eq + Send + Sync + Debug {}

// automatically impl the trait for all types that fulfill the condition/required traits
impl<T> CrackTarget for T where T: 'static + Eq + Send + Sync + Debug {}

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
    hash_type_to_str_repr_fn: fn(hash: &T) -> String
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
        hash_type_to_str_repr_fn: fn(hash: &T) -> String
    ) -> Self {
        let target_hash = match target_hash {
            TargetHashInput::HashAsStr(hash_str) => hash_str_repr_to_hash_type_fn(hash_str),
            TargetHashInput::Plaintext(input) => hash_fn(input)
        };
        Self {
            target_hash,
            hash_fn,
            hash_str_repr_to_hash_type_fn,
            hash_type_to_str_repr_fn
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
    pub fn target_hash(&self) -> &T {
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
