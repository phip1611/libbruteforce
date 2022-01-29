//! This library helps you to brute force hashes (e.g. passwords). It includes a set of pre-configured
//! hashing functions, like md5 or sha256. You can also provide your own hashing function. PLEASE DO NOT
//! use this software to harm someones privacy in any kind! This project was made for fun and for teaching myself
//! new things about Rust.
//!
//! # Minimal example
//! ```rust
//!
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
pub use crack::parameter::CrackParameter;
use std::fmt::{Debug, Formatter};

mod crack;
#[cfg(test)]
mod testutil;

// Public API
pub mod symbols;
pub mod transform_fns;

/// Common trait for crack targets (hashes or plain text to crack). This is the super-type
/// which enables the usage of multiple hashing algorithms. An example that
/// implements this/fulfils the trait requirements is [`String`].
// 'static:
//  - it means the type does not contain any non-static references; i.e. consumes can
//    own implementers of this type easily
//  - https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html
pub trait CrackTarget: 'static + Eq + Send + Sync + Debug + Clone {}

// automatically impl the trait for all types that fulfill the condition/required traits
impl<T> CrackTarget for T where T: 'static + Eq + Send + Sync + Debug + Clone {}

/// Generic trait to generalize multiple implementations of [`TargetHashAndHashFunction`].
/// This allows a friendly API for a dynamic selection of different hashing algorithms
/// during runtime.
pub trait TargetHashAndHashFunctionTrait<T: CrackTarget> {
    /// Takes the raw string representation of a hash and transforms it to the
    /// target type. Afterwards, it can be compared against values transformed
    /// with [`Self::transform`].
    fn hash_str_to_target_type(&self, hash: &str) -> T;
    /// Calculates the hash of the given input string.
    fn transform(&self, input: &str) -> T;
    /// Returns the target hash value that we want to crack.
    /// This can be a sha256 hash for example in the target type
    /// representation that the hashing library uses.
    fn get_target(&self) -> &T;
    /// Transforms the input using [`Self::transform`] and checks
    /// if is equal to [`Self::target`]. To check for equality, the [`Eq`]
    /// implementation of `T` which os of type0 [`CrackTarget`] gets used.
    fn hash_matches(&self, input: &str) -> bool {
        &self.transform(input) == self.get_target()
    }
}

/// Abstraction over a hashing algorithm and the target hash that needs to be cracked.
/// `T` is of type [`CrackTarget`]. Multiple implementations can be generalized with
/// the trait [`TargetHashAndHashFunctionTrait`].
pub struct TargetHashAndHashFunction<T: CrackTarget> {
    /// The target hash we want to crack.
    target: T,
    /// Function that calculates the hash of the given input.
    transform_fn: fn(input: &str) -> T,
    /// Function that transforms a `T` in string representation to a real `T`.
    /// For example, this transforms a `sha256` string representation to the runtime
    /// type the hashing library uses.
    target_str_repr_to_target_type_fn: fn(hash_as_string: &str) -> T,
}

impl<T: CrackTarget> TargetHashAndHashFunction<T> {
    /// Constructor that takes a hashing function and a target hash.
    ///
    /// # Parameters
    /// * `target_hash` String representation of the target hash we want to crack.
    ///                 This is usually the hex string representation of a sha256 hash or so.
    /// * `transform_fn` Transforms a plain input password/guess of type `str` to the target hash.
    ///                  This is the hashing function.
    /// * `target_str_repr_to_target_type_fn` Function that can take the argument `target_hash`
    ///                                       and transform it to the target hashing type. This
    ///                                       usually transforms the hex string that represents the
    ///                                       hash to bytes in memory.
    pub fn new(
        target_hash: &str,
        transform_fn: fn(&str) -> T,
        target_str_repr_to_target_type_fn: fn(hash_as_string: &str) -> T,
    ) -> Self {
        Self {
            target: target_str_repr_to_target_type_fn(target_hash),
            transform_fn,
            target_str_repr_to_target_type_fn,
        }
    }
}

impl<T: CrackTarget> Debug for TargetHashAndHashFunction<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TargetHashAndHashFunction")
            .field("target", &self.target)
            .field("transform_fn", &"<func impl>")
            .finish()
    }
}

impl<T: CrackTarget> TargetHashAndHashFunctionTrait<T> for TargetHashAndHashFunction<T> {
    fn hash_str_to_target_type(&self, hash_as_string: &str) -> T {
        (self.target_str_repr_to_target_type_fn)(hash_as_string)
    }

    fn transform(&self, input: &str) -> T {
        (self.transform_fn)(input)
    }

    fn get_target(&self) -> &T {
        &self.target
    }
}
