use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crate::indices::{indices_create, indices_increment_by, indices_to_string};
use crate::symbols::combinations_count;
use crate::transformation_fns::TransformationFn;

mod indices;
pub mod symbols;
pub mod transformation_fns;

/// This function takes a target string( e.g. a MD5-Hash), the alphabet, the max length and tries
/// to find the combination resulting in the target (the password).
///
/// You can specify the alphabet that should be used.
///
/// You can supply a transform function that transforms every possible value before it
/// is matched with the target. This transform function can be the identity, a
/// hashing algorithm, a hashing algorithm with appended salt to the value or something else.
///
/// This function is multi threaded. Therefore it wants to take ownership of all variables to
/// prevent memory lifetimes issues.
pub fn crack(target: String,
             alphabet: Box<[char]>,
             max_length: usize,
             transform_fn: TransformationFn) -> Option<String> {
    if max_length == 0 {
        panic!("Max length must be >= 1!");
    }

    // only do multiple threads for big workloads
    let thread_count = if combinations_count(&alphabet, max_length as u32) >= 10000 {
        num_cpus::get() as isize
    } else { 1 };

    // make function parameters ready for sharing between threads
    let alphabet = Arc::from(alphabet);
    let target: Arc<String> = Arc::from(target);
    let done = Arc::from(AtomicBool::from(false));

    let mut handles = vec![];

    // for each thread (preparation + creation + start)
    for tid in 0..thread_count {
        // spawn thread for each cpu
        let mut indices = indices_create(max_length);

        // variables needed in thread
        let target = Arc::clone(&target);
        let alphabet = Arc::clone(&alphabet);

        let done = done.clone();

        // prepare array for thread with right starting index
        indices_increment_by(&alphabet, &mut indices, tid as isize).expect("Increment failed");

        // spawn all threads
        let h = thread::spawn(move || {
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
        });
        handles.push(h);
    }

    let mut result = None;

    // auf alle Threads warten
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
    use crate::transformation_fns::identity::NO_HASHING;
    use crate::transformation_fns::sha256::SHA256_HASHING;

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
