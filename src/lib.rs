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
//! let result = crack(target.clone(), alphabet, input.len(), transformation_fns::SHA256_HASHING);
//! ```

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crate::indices::{indices_create, indices_increment_by, indices_to_string};
use crate::symbols::combinations_count;
use crate::transformation_fns::TransformationFn;
use std::thread::JoinHandle;

mod indices;
pub mod symbols;
pub mod transformation_fns;

/// This function starts a multithreaded brute force attack on a given target string. It supports
/// any alphabet you want to use. You must provide a transformation function. There is a pre-build
/// set of transformation functions available, such as `transformation_fns::NO_HASHING`, or
/// `transformation_fns::SHA256`. You can also provide your own function if it is compatible
/// with `transformation_fns::TransformationFn`.
pub fn crack(target: String,
             alphabet: Box<[char]>,
             max_length: usize,
             transform_fn: TransformationFn) -> Option<String> {
    if max_length == 0 {
        panic!("Max length must be >= 1!");
    }

    // only do multiple threads for big workloads
    let thread_count = if combinations_count(&alphabet, max_length as u32) >= 10_000 {
        num_cpus::get() as isize
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
        max_length
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

/// Spawns all worker threads.
fn spawn_worker_threads(done: Arc<AtomicBool>,
                        target: Arc<String>,
                        transform_fn: TransformationFn,
                        alphabet: Arc<Box<[char]>>,
                        thread_count: isize,
                        max_length: usize) -> Vec<JoinHandle<Option<String>>> {
    let mut handles = vec![];
    for tid in 0..thread_count {
        // spawn thread for each cpu
        let mut indices = indices_create(max_length);

        // variables needed in thread
        let target = Arc::clone(&target);
        let alphabet = Arc::clone(&alphabet);

        let done = done.clone();

        // prepare array for thread with right starting index
        indices_increment_by(&alphabet, &mut indices, tid as isize).expect("Increment failed");

        handles.push(
            spawn_worker_thread(
                done,
                target,
                transform_fn,
                indices,
                alphabet,
                thread_count
            )
        );
    }
    handles
}

/// Spawns a worker thread with its work loop.
fn spawn_worker_thread(done: Arc<AtomicBool>,
                       target: Arc<String>,
                       transform_fn: TransformationFn,
                       indices: Box<[isize]>,
                       alphabet: Arc<Box<[char]>>,
                       thread_count: isize) -> JoinHandle<Option<String>> {
    // mark var as mutable for compiler
    let mut indices = indices;
    thread::spawn(move || {
        // The result that the thread calculated/found
        let mut result = None;

        /// The number after how many iterations the thread looks if another thread
        /// is already done, so that we can stop further work. We do this only after
        /// a few millions iterations to keep the overhead low. Tests on my machine
        /// showed that 1 million iterations take about 1.6s - this should be okay
        /// because the overhead is not that big
        const INTERRUPT_COUNT_THRESHOLD: usize = 1_000_000;
        let mut interrupt_count = INTERRUPT_COUNT_THRESHOLD;

        // infinite incrementing; break inside loop if its the right time for
        loop {
            if interrupt_count > 0 {
                interrupt_count -= 1;
            } else {
                interrupt_count = INTERRUPT_COUNT_THRESHOLD;
                let done = done.load(Ordering::SeqCst);
                if done {
                    // another thread already found a solution
                    break;
                }
            }

            let res = indices_increment_by(&alphabet, &mut indices, thread_count);
            if res.is_err() {
                // reached incrementing limit; thread is done
                break;
            }

            let string = indices_to_string(&alphabet, &indices);
            // transform; e.g. hashing
            let transformed_string = transform_fn(&string);
            if transformed_string == *target {
                // let other threads now we are done
                done.store(true, Ordering::SeqCst);
                result = Some(string);
            }
        }
        result
    })
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
        let result = crack(target.clone(), alphabet, input.len(), NO_HASHING);
        assert!(target.eq(&result.unwrap()), "target and cracked result must equal!");
    }

    #[test]
    fn test_crack_sha256() {
        let alphabet = full_alphabet();
        let input = String::from("a+c");
        let target = SHA256_HASHING(&input);
        let result = crack(target.clone(), alphabet, input.len(), SHA256_HASHING);
        assert!(result.is_some(), "a solution MUST be found");
        let result = result.unwrap();
        assert!(input.eq(&result), "target and cracked result must equal!");
    }
}
