//! This library helps you to brute force hashes (e.g. passwords). It includes a set of pre-configured
//! hashing functions, like md5 or sha256. You can also provide your own hashing function. PLEASE DO NOT
//! use this software to harm someones privacy in any kind! This project was made for fun and for teaching myself
//! new things about Rust.
//! # Usage
//!
//! ```
//! use libbruteforce::{symbols, transform_fns};
//! use libbruteforce::CrackParameter;
//! use simple_logger::SimpleLogger;
//!
//! fn main() {
//!     // to get information about trace! logs (like progress) on the console
//!     SimpleLogger::new().init().unwrap();
//!
//!     let alphabet = symbols::Builder::new().with_lc_letters().with_common_special_chars().build();
//!     let sha256_hash = String::from("3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d");
//!     // sha256("a+c")
//!     let res = libbruteforce::crack(
//!         CrackParameter::new(
//!             sha256_hash.clone(), alphabet, 3, 0, transform_fns::SHA256_HASHING, true,
//!         )
//!     );
//!     if res.is_success() {
//!         println!("Password is: {}", res.solution.unwrap())
//!     }
//! }
//! ```

pub use crack::crack;
pub use crack::parameter::CrackParameter;

mod crack;
mod util;

// Public API
pub mod symbols;
pub mod transform_fns;
