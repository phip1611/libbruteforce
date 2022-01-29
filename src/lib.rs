//! This library helps you to brute force hashes (e.g. passwords). It includes a set of pre-configured
//! hashing functions, like md5 or sha256. You can also provide your own hashing function. PLEASE DO NOT
//! use this software to harm someones privacy in any kind! This project was made for fun and for teaching myself
//! new things about Rust.
//!
//! # Minimal example
//! ```rust
//! use libbruteforce::{BasicCrackParameter, CrackParameter, TargetHashInput};
//! use libbruteforce::hash_fncs::sha256_hashing;
//! use libbruteforce::symbols;
//!
//! // sha256("a+c")
//! let sha256_hash = "3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d";
//! let max_len = 3;
//! let min_len = 0;
//! let alphabet = symbols::Builder::new().with_digits().build();
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

use crate::crack::worker_threads::spawn_worker_threads;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

pub use crack::CrackResult;
pub use parameter::*;

mod crack;
#[cfg(test)]
mod testutil;

// Public API
pub mod hash_fncs;
mod parameter;
pub mod symbols;

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

/// This function starts a multi-threaded brute force attack on a given target string. It supports
/// any alphabet you would like to use. You must provide a hashing function. There is a pre-build
/// set of transformation functions available, such as [`hash_fncs::no_hashing`] or
/// [`hash_fncs::sha256_hashing`]. You can also provide your own hashing strategy.
///
/// This library is really "dumb". It checks each possible value and doesn't use any probabilities
/// for more or less probable passwords.
///
/// # Parameters
/// * `param` - See [`CrackParameter`]
///
/// # Return
/// Returns a [`CrackResult`].
pub fn crack<T: CrackTarget>(param: CrackParameter<T>) -> CrackResult {
    let param = InternalCrackParameter::from(param);
    let param = Arc::from(param);

    // shared atomic bool so that all threads can look if one already found a solution
    // so they can stop their work. This only gets checked at every millionth iteration
    // for better performance.
    let done = Arc::from(AtomicBool::from(false));
    let instant = Instant::now();
    let handles = spawn_worker_threads(param.clone(), done);

    // wait for all threads
    let solution = handles
        .into_iter()
        .map(|h| h.join().unwrap()) // result of the Option<String> from the threads
        .flatten()
        .last(); // extract from the collection

    let seconds = instant.elapsed().as_secs_f64();

    let param =
        Arc::try_unwrap(param).unwrap_or_else(|_| panic!("There should only be one reference!"));
    if let Some(solution) = solution {
        CrackResult::new_success(param, seconds, solution)
    } else {
        CrackResult::new_failure(param, seconds)
    }
}

#[cfg(test)]
mod tests {
    use crate::hash_fncs::no_hashing;
    use crate::testutil::{
        create_test_crack_params_full_alphabet, create_test_crack_params_full_alphabet_sha256,
        create_test_crack_params_full_alphabet_sha256_fair,
    };
    use crate::{BasicCrackParameter, CrackParameter, TargetHashInput};

    use super::*;

    #[test]
    #[should_panic]
    fn test_crack_should_panic_1() {
        let cp = CrackParameter::new(
            BasicCrackParameter::new(vec!['a'].into_boxed_slice(), 4, 5, false),
            no_hashing(TargetHashInput::Plaintext("a+c")),
        );
        // expect panic; min > max
        let _res = crack(cp);
    }

    #[test]

    fn test_crack_dont_find_bc_of_min_length() {
        let cp = CrackParameter::new(
            BasicCrackParameter::new(vec!['a'].into_boxed_slice(), 4, 3, false),
            no_hashing(TargetHashInput::Plaintext("a+c")),
        );
        // expect panic; min > max
        let res = crack(cp);
        assert!(
            !res.is_success(),
            "should not find result, because of min length!"
        );
    }

    #[test]
    fn test_crack_identity() {
        let input = String::from("a+c");
        let target = input.clone(); // identity hashing
        let cp = create_test_crack_params_full_alphabet(&target);
        let res = crack(cp);
        assert!(res.is_success(), "a solution must be found!");
        assert!(
            input.eq(res.solution().as_ref().unwrap()),
            "target and cracked result must equal!"
        );
    }

    /// Warning: can run a few seconds (15-30)
    #[test]
    fn test_crack_sha256_fair_and_unfair_mode() {
        let input = "a+c";
        let target =
            String::from("3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d");

        let cp = create_test_crack_params_full_alphabet_sha256(&target);
        let cp_fair = create_test_crack_params_full_alphabet_sha256_fair(&target);

        let res = crack(cp);
        assert!(res.is_success(), "A solution MUST be found!");
        assert!(
            input.eq(res.solution().as_ref().unwrap()),
            "The cracked value is wrong! It's"
        );

        if num_cpus::get() > 1 {
            assert!(cp_fair.fair_mode(), "Fair mode must be activated"); // check if really multiple threads are used
            assert!(res.thread_count() > 1, "multiple threads must be used"); // check if really multiple threads are used

            let res_fair = crack(cp_fair);
            assert!(res_fair.is_success(), "A solution MUST be found!");
            assert!(
                input.eq(res.solution().as_ref().unwrap()),
                "The cracked value is wrong!"
            );
            assert!(
                res.thread_count() > res_fair.thread_count(),
                "fair mode must use less treads!"
            );
        }
    }
}
