//! This library helps you to brute force hashes (e.g. passwords). It includes a set of pre-configured
//! hashing functions, like md5 or sha256. You can also provide your own hashing function. PLEASE DO NOT
//! use this software to harm someones privacy in any kind! This project was made for fun and for teaching myself
//! new things about Rust.
//!
//! # Minimal example
//! ```rust
//! use simple_logger::SimpleLogger;
//! use libbruteforce::{symbols};
//! use libbruteforce::CrackParameter;
//! use libbruteforce::transform_fns::{SHA256_HASHING, str_to_sha256_hash};
//!
//! // Minimal example.
//!
//! // to get information about trace! logs (like progress) on the console
//! SimpleLogger::new().init().unwrap();
//!
//! let alphabet = symbols::Builder::new().with_lc_letters().with_common_special_chars().build();
//!
//! // sha256("a+c")
//! let sha256_hash = str_to_sha256_hash("3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d");
//!
//! // the actual cracking
//! let res = libbruteforce::crack(
//!     CrackParameter::new(
//!         sha256_hash.clone(), alphabet, 3, 0, SHA256_HASHING, true,
//!     )
//! );
//!
//! if let Some(solution) = res.solution {
//!     println!("Password is: {}", solution);
//!     println!("Took {}s", res.seconds_as_fraction);
//! }
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
use std::fmt::Debug;

mod crack;
mod util;

// Public API
pub mod symbols;
pub mod transform_fns;

/// Common trait for crack targets. This is the super-type for all hashes one
/// want to track, i.e. SHA-1 or SHA-256. This can also refer to a plain
/// string.
pub trait CrackTarget: 'static + Eq + Send + Sync + Debug {}

// automatically impl the trait for all types that fulfill the condition/required traits
impl<T> CrackTarget for T where T: 'static + Eq + Send + Sync + Debug {}
