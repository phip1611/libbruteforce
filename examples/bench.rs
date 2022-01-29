//! Starts a cracking process that does the maximum number of iterations for
//! a alphabet and a specified length. Can be used to compare systems and to
//! check the effect of code changes/improvements.
//!
//! PS: RUN THIS IN RELEASE MODE (= a lot faster). `cargo run --bin bench --release`

use libbruteforce::hash_fncs::{sha256_hashing, Sha256Hash};
use libbruteforce::symbols::{combinations_count, Builder};
use libbruteforce::{BasicCrackParameter, CrackParameter, TargetHashInput};
use sha2::Sha256;
use simple_logger::SimpleLogger;

fn main() {
    // to get information about trace! logs (like progress) on the console
    SimpleLogger::new().with_utc_timestamps().init().unwrap();

    // create value we want to hash
    const MAX_LEN: u32 = 4; // everything above 4 with the full alphabet => takes already quite some time
    let alphabet = Builder::new().full().build();
    let worst_case_pw = create_worst_case_search_password(&alphabet, MAX_LEN);

    println!(
        "Start benchmark with {} possible combinations",
        combinations_count(&alphabet, MAX_LEN, 0)
    );
    println!("PLEASE MAKE SURE THAT YOU RUN THIS BIN IN RELEASE MODE..OTHERWISE IT TAKES AGES :)");
    let sha256_hashing = sha256_hashing(TargetHashInput::Plaintext(&worst_case_pw));
    println!(
        "Trying to crack '{}'. SHA256 is '{:?}'",
        worst_case_pw, sha256_hashing.hash_type_to_str_repr(sha256_hashing.target_hash())
    );

    // the actual cracking
    let crack_res = libbruteforce::crack(CrackParameter::new(
        BasicCrackParameter::new(alphabet, MAX_LEN, 0, false),
        sha256_hashing,
    ));


    assert!(
        crack_res.is_success(),
        "A solution MUST be found! Should be '{}'",
        worst_case_pw
    );
    assert!(
        worst_case_pw.eq(crack_res.solution().as_ref().unwrap()),
        "The solution MUST be correct!"
    );

    // output stats
    println!(
        "Found worst case solution for given alphabet with max len = {}",
        MAX_LEN
    );
    println!("Solution is: '{}'", crack_res.solution().as_ref().unwrap());
    println!(
        "Did {} iterations in {} threads in {:>7.3}s",
        crack_res.combinations_total(),
        crack_res.thread_count(),
        crack_res.seconds_as_fraction()
    );
    let iterations_ps = crack_res.combinations_total() as f64 / crack_res.seconds_as_fraction();
    let iterations_ps_pt = iterations_ps / crack_res.thread_count() as f64;
    let m_iterations_ps = iterations_ps / 1_000_000.0;
    let m_iterations_ps_pt = iterations_ps_pt / 1_000_000.0;
    println!("{:>7.3} million iterations / s ", m_iterations_ps);
    println!(
        "{:>7.3} million iterations / s (per thread)",
        m_iterations_ps_pt
    );
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
