//! This library helps you to brute force hashes (e.g. passwords). It includes a set of pre-configured
//! hashing functions, like md5 or sha256. You can also provide your own hashing function. PLEASE DO NOT
//! use this software to harm someones privacy in any kind! This project was made for fun and for teaching myself
//! new things about Rust.
//! # Usage
//!
//! ```
//! use libbruteforce::{symbols, crack};
//! use libbruteforce::transformation_fns;
//!
//! let alphabet = symbols::full_alphabet();
//! // or let alphabet = symbols::build_alphabet(true, true, false, false, false, false, false)
//! let input = String::from("a+c");
//! let target = String::from("3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d");
//!
//! let result = crack(target, alphabet, input.len(), transformation_fns::SHA256_HASHING, false);
//! ```

use crate::symbols::combinations_count;
use crate::transformation_fns::TransformationFn;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use crate::worker_threads::spawn_worker_threads;

pub mod benchmark;
mod indices;
mod worker_threads;
pub mod symbols;
pub mod transformation_fns;

/// This function starts a multithreaded brute force attack on a given target string. It supports
/// any alphabet you want to use. You must provide a transformation function. There is a pre-build
/// set of transformation functions available, such as `transformation_fns::NO_HASHING`, or
/// `transformation_fns::SHA256`. You can also provide your own function if it is compatible
/// with `transformation_fns::TransformationFn`.
///
/// * `fair_mode` - use n - 1 (instead of n) threads to keep your system useable (n: #cores)
pub fn crack(target: String,
             alphabet: Box<[char]>,
             max_length: usize,
             transform_fn: TransformationFn,
             fair_mode: bool) -> Option<String> {
    if max_length == 0 {
        panic!("Max length must be >= 1!");
    }

    // only do multiple threads for big workloads
    let thread_count = if combinations_count(&alphabet, max_length as u32) >= 10_000 {
        if fair_mode {
            num_cpus::get()
        } else {
            (num_cpus::get() - 1)
        }
    } else { 1 };

    // make function parameters ready for sharing between threads
    let alphabet = Arc::from(alphabet);
    let target: Arc<String> = Arc::from(target);
    let done = Arc::from(AtomicBool::from(false));

    let handles = spawn_worker_threads(
        done,
        target,
        transform_fn,
        alphabet,
        thread_count,
        max_length,
    );
    let mut result = None;

    // wait for all threads
    handles.into_iter().for_each(|h| {
        if let Some(x) = h.join().unwrap() {
            result = Some(x);
        }
    });

    result
}

#[cfg(test)]
mod tests {
    use crate::symbols::full_alphabet;
    use crate::transformation_fns::NO_HASHING;
    use crate::transformation_fns::SHA256_HASHING;

    use super::*;

    #[test]
    fn test_crack_identity() {
        let alphabet = full_alphabet();
        let input = String::from("a+c");
        let target = input.clone();
        let result = crack(
            target.clone(),
            alphabet,
            input.len(),
            NO_HASHING,
            false,
        );
        assert!(target.eq(&result.unwrap()), "target and cracked result must equal!");
    }

    #[test]
    fn test_crack_sha256() {
        let alphabet = full_alphabet();
        let input = String::from("a+c");
        let target = SHA256_HASHING(&input);
        let result = crack(
            target.clone(),
            alphabet,
            input.len(),
            SHA256_HASHING,
            false,
        );
        assert!(result.is_some(), "a solution MUST be found");
        let result = result.unwrap();
        assert!(input.eq(&result), "target and cracked result must equal!");
    }
}
