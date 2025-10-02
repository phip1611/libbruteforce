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
//! This library helps you to brute force hashes (e.g. passwords). It includes a set of
//! pre-configured hashing functions, like md5 ([`crate::hash_fncs::md5_hashing`]),
//! sha1 ([`crate::hash_fncs::sha1_hashing`]), or sha256 ([`crate::hash_fncs::sha256_hashing`]).
//! You can also provide your own hashing function. PLEASE DO NOT use this software to harm
//! someones privacy in any kind! This project was made for fun and for teaching myself new
//! things about Rust.
//!
//! The main function of this crate is [`crack<T: CrackTarget>()`].
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
    clippy::must_use_candidate,
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

use crate::crack::worker_threads::spawn_worker_threads;
use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, RecvError, RecvTimeoutError};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

pub use crack::CrackResult;
pub use parameter::*;

mod crack;
#[cfg(test)]
mod testutil;

// Public API
pub mod hash_fncs;
mod parameter;
pub mod symbols;

/// Common trait for crack targets (hashes or plain text to crack).
///
/// This is the super-type which enables the usage of multiple hashing algorithms.
/// An example that implements this/fulfils the trait requirements is [`String`].
// 'static:
//  - it means the type does not contain any non-static references; i.e. consumes can
//    own implementers of this type easily
//  - https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html
pub trait CrackTarget: 'static + Eq + Send + Sync + Debug + Clone {}

// automatically impl the trait for all types that fulfill the condition/required traits
impl<T> CrackTarget for T where T: 'static + Eq + Send + Sync + Debug + Clone {}

/// Start a multi-threaded brute force attack on a given target string.
///
/// It supports any alphabet you would like to use. You must provide a hashing function.
/// There is a pre-build set of transformation functions available, such as [`hash_fncs::no_hashing`] or
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
    // solutions are send over the channel
    // when the sender channels are closed, the threads haven't found a solution
    let (sender, receiver) = channel();

    let instant = Instant::now();
    let handles = spawn_worker_threads(param.clone(), sender, done.clone());

    let solution = match receiver.recv() {
        Ok(solution) => Some(solution),
        Err(RecvError) => None,
    };
    done.store(true, Ordering::Relaxed);
    // if we don't drop the receiver, the threads will keep searching
    drop(receiver);

    let seconds = instant.elapsed().as_secs_f64();

    // wait for all threads to actually finish
    handles.into_iter().for_each(|h| h.join().unwrap());

    let param =
        Arc::try_unwrap(param).unwrap_or_else(|_| panic!("There should only be one reference!"));
    solution.map_or_else(
        || CrackResult::new_failure(&param, seconds),
        |solution| CrackResult::new_success(&param, seconds, solution),
    )
}

/// Starts a multi-threaded brute force attack on a given target string that keeps search after a match.
///
/// This variant of [`crack()`] is useful for weak hash functions, where you need the original value
/// but there are many matches for a hash.
///
/// It supports any alphabet you would like to use. You must provide a hashing function.
/// There is a pre-build set of transformation functions available, such as [`hash_fncs::no_hashing`] or
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
pub fn crack_iter<T: CrackTarget>(param: CrackParameter<T>) -> CrackResults<T> {
    let param = InternalCrackParameter::from(param);
    let param = Arc::from(param);

    // shared atomic bool so that all threads can look if one already found a solution
    // so they can stop their work. This only gets checked at every millionth iteration
    // for better performance.
    let done = Arc::from(AtomicBool::from(false));
    // solutions are send over the channel
    // when the sender channels are closed, the threads haven't found a solution
    let (sender, receiver) = channel();

    let start = Instant::now();
    let handles = spawn_worker_threads(param.clone(), sender, done.clone());

    CrackResults {
        receiver,
        done,
        handles,
        start,
        param,
    }
}

#[derive(Debug)]
pub struct CrackResults<T: CrackTarget> {
    receiver: Receiver<String>,
    done: Arc<AtomicBool>,
    handles: Vec<JoinHandle<()>>,
    start: Instant,
    param: Arc<InternalCrackParameter<T>>,
}

impl<T: CrackTarget> Iterator for CrackResults<T> {
    type Item = CrackResult;

    /// This call is blocking.
    ///
    /// For the non-blocking variant, use
    /// [`CrackResults::next_timeout()`]
    fn next(&mut self) -> Option<Self::Item> {
        match self.receiver.recv() {
            Ok(solution) => {
                let seconds = self.start.elapsed().as_secs_f64();
                Some(CrackResult::new_success(&self.param, seconds, solution))
            }
            Err(RecvError) => {
                // all senders are closed so this shouldn't be necessary, but just in case
                self.done.store(true, Ordering::Relaxed);
                // cleanup all the handles
                while let Some(handle) = self.handles.pop() {
                    handle.join().unwrap();
                }
                None
            }
        }
    }
}

impl<T: CrackTarget> CrackResults<T> {
    /// Attempts to wait for a result, returning `Ok(None)` if there is still progress.
    ///
    /// Will return `Err(CrackResult)` if all the threads are done.
    pub fn next_timeout(&mut self, timeout: Duration) -> Result<Option<CrackResult>, CrackResult> {
        match self.receiver.recv_timeout(timeout) {
            Ok(solution) => {
                let seconds = self.start.elapsed().as_secs_f64();
                Ok(Some(CrackResult::new_success(
                    &self.param,
                    seconds,
                    solution,
                )))
            }
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => {
                // all senders are closed so this shouldn't be necessary, but just in case
                self.done.store(true, Ordering::Relaxed);
                // cleanup all the handles
                while let Some(handle) = self.handles.pop() {
                    handle.join().unwrap();
                }

                let seconds = self.start.elapsed().as_secs_f64();
                Err(CrackResult::new_failure(&self.param, seconds))
            }
        }
    }

    /// Stop search for a match
    pub fn stop(&mut self) {
        // stop all the threads
        self.done.store(true, Ordering::Relaxed);
        // cleanup all the handles
        while let Some(handle) = self.handles.pop() {
            handle.join().unwrap();
        }
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
        let cp = create_test_crack_params_full_alphabet(&input);
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
            input.eq(res.solution().unwrap()),
            "The cracked value is wrong! It's"
        );

        if get_thread_count(false) > 1 {
            assert!(cp_fair.fair_mode(), "Fair mode must be activated"); // check if really multiple threads are used
            assert!(res.thread_count() > 1, "multiple threads must be used"); // check if really multiple threads are used

            let res_fair = crack(cp_fair);
            assert!(res_fair.is_success(), "A solution MUST be found!");
            assert!(
                input.eq(res.solution().unwrap()),
                "The cracked value is wrong!"
            );
            assert!(
                res.thread_count() > res_fair.thread_count(),
                "fair mode must use less treads!"
            );
        }
    }
}
