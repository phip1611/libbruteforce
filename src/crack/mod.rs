use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

use crate::CrackTarget;
use parameter::CrackParameter;
use parameter::InternalCrackParameter;
use result::CrackResult;
use worker_threads::spawn_worker_threads;

// Public API
pub mod parameter;

mod indices;
mod result;
mod worker_threads;

/// This function starts a multithreaded brute force attack on a given target string. It supports
/// any alphabet you want to use. You must provide a transformation function. There is a pre-build
/// set of transformation functions available, such as `transform_fns::NO_HASHING`, or
/// `transform_fns::SHA256`. You can also provide your own function if it is compatible
/// with TODO.
///
/// This library is really "dumb". It checks each possible value and doesn't use any probabilities
/// for more or less probable passwords.
///
/// # Parameters
/// * `cp` - see [`CrackParameter`]
pub fn crack<T: CrackTarget>(cp: CrackParameter<T>) -> CrackResult<T> {
    let cp = InternalCrackParameter::from(cp);
    let cp = Arc::from(cp);

    // shared atomic bool so that all threads can look if one already found a solution
    // so they can stop their work. This only gets checked at every millionth iteration
    // for better performance.
    let done = Arc::from(AtomicBool::from(false));
    let instant = Instant::now();
    let handles = spawn_worker_threads(cp.clone(), done);

    // wait for all threads
    let solution = handles
        .into_iter()
        .map(|h| h.join().unwrap()) // result of the Option<String> from the threads
        .flatten()
        .last(); // extract from the collection

    let seconds = instant.elapsed().as_secs_f64();

    let cp = Arc::try_unwrap(cp).unwrap_or_else(|_| panic!("There should only be one reference!"));
    if let Some(solution) = solution {
        CrackResult::new_success(cp, seconds, solution)
    } else {
        CrackResult::new_failure(cp, seconds)
    }
}

#[cfg(test)]
mod tests {
    use crate::testutil::{
        create_test_crack_params_full_alphabet, create_test_crack_params_full_alphabet_sha256,
        create_test_crack_params_full_alphabet_sha256_fair,
    };
    use crate::transform_fns::{no_hashing};

    use super::*;

    #[test]
    #[should_panic]
    fn test_crack_should_panic_1() {
        let input = "a+c";
        let cp = CrackParameter::new(no_hashing(input), vec!['a'].into_boxed_slice(), 4, 5, false);
        // expect panic; min > max
        let _res = crack(cp);
    }

    #[test]

    fn test_crack_dont_find_bc_of_min_length() {
        let input = "a+c";
        let cp = CrackParameter::new(no_hashing(input), vec!['a'].into_boxed_slice(), 4, 3, false);
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
