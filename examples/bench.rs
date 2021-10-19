//! Starts a cracking process that does the maximum number of iterations for
//! a alphabet and a specified length. Can be used to compare systems and to
//! check the effect of code changes/improvements.
//!
//! PS: RUN THIS IN RELEASE MODE (= a lot faster). `cargo run --bin bench --release`

use libbruteforce::symbols::{
    combinations_count,
    Builder,
};
use libbruteforce::transform_fns::{
    sha256_hash_to_hex_string,
    SHA256_HASHING,
};
use libbruteforce::CrackParameter;
use simple_logger::SimpleLogger;

fn main() {
    // to get information about trace! logs (like progress) on the console
    SimpleLogger::new().init().unwrap();

    // create value we want to hash
    const MAX_LEN: u32 = 4; // everything above 4 with the full alphabet => takes already quite some time
    let alphabet = Builder::new().full().build();
    let target = create_worst_case_search_password(&alphabet, MAX_LEN);
    let target_hash = SHA256_HASHING(&target);

    println!(
        "Start benchmark with {} possible combinations",
        combinations_count(&alphabet, MAX_LEN, 0)
    );
    println!("PLEASE MAKE SURE THAT YOU RUN THIS BIN IN RELEASE MODE..OTHERWISE IT TAKES AGES :)");
    println!(
        "Trying to crack '{}'. SHA256 is '{}'",
        target,
        sha256_hash_to_hex_string(&target_hash)
    );

    // the actual cracking
    let cp = CrackParameter::new(
        target_hash.clone(),
        alphabet,
        MAX_LEN,
        0,
        SHA256_HASHING,
        // without system gets mostly unusable
        // set this only to false if you
        true,
    );
    let result = libbruteforce::crack(cp);

    assert!(
        result.is_success(),
        "A solution MUST be found! Should be '{}'",
        target
    );
    assert!(
        target.eq(result.solution.as_ref().unwrap()),
        "The solution MUST be correct!"
    );

    // output stats
    println!(
        "Found worst case solution for given alphabet with max len = {}",
        MAX_LEN
    );
    println!("Solution is: '{}'", result.solution.unwrap());
    println!(
        "Did {} iterations in {} threads in {}s",
        result.combinations_total, result.thread_count, result.seconds_as_fraction
    );
    let iterations_p_s = result.combinations_total as f64 / result.seconds_as_fraction;
    let iterations_ps_pt = iterations_p_s / result.thread_count as f64;
    println!("{} iterations / s ", iterations_p_s as usize);
    println!("{} iterations / s (per thread)", iterations_ps_pt as usize);
}

/// Returns the worst-case search password for the given alphabet.
fn create_worst_case_search_password(alphabet: &[char], len: u32) -> String {
    let mut target = String::new();
    let max_index = alphabet.len() - 1;
    for _ in 0..len {
        target.push(alphabet[max_index]);
    }
    target
}
