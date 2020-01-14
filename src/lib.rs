//! This library helps you to brute force hashes (e.g. passwords). It includes a set of pre-configured
//! hashing functions, like md5 or sha256. You can also provide your own hashing function. PLEASE DO NOT
//! use this software to harm someones privacy in any kind! This project was made for fun and for teaching myself
//! new things about Rust.
//! # Usage
//!
//! ```
//! use libbruteforce::{crack, transform_fns, symbols, CrackParameter};
//!
//! let alphabet = symbols::full_alphabet();
//! // or let alphabet = symbols::build_alphabet(true, true, false, false, false, false, false)
//! let input = String::from("a+c");
//! let sha256_hash = String::from("3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d");
//! let cp = CrackParameter::new(sha256_hash.clone(), alphabet, 3, 0, transform_fns::SHA256_HASHING, true);
//! let res = crack(cp);
//! if res.is_success() { let sol = res.solution.unwrap(); }
//! ```

pub use crack::crack;
pub use crack::parameter::CrackParameter;

mod crack;
mod util;

// Public PAI
pub mod symbols;
pub mod transform_fns;

