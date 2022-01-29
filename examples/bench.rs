//! Starts a cracking process that does the maximum number of iterations for
//! a alphabet and a specified length. Can be used to compare systems and to
//! check the effect of code changes/improvements.
//!
//! PS: RUN THIS IN RELEASE MODE (= a lot faster). `cargo run --bin bench --release`

use libbruteforce::symbols::{combinations_count, Builder};
use libbruteforce::transform_fns::{sha256_hashing, Sha256Hash};
use libbruteforce::CrackParameter;
use sha2::Sha256;
use simple_logger::SimpleLogger;

fn main() {
    // to get information about trace! logs (like progress) on the console
    SimpleLogger::new().with_utc_timestamps().init().unwrap();

    // create value we want to hash
    const MAX_LEN: u32 = 4; // everything above 4 with the full alphabet => takes already quite some time
    let alphabet = Builder::new().full().build();
    let worst_case_pw = create_worst_case_search_password(&alphabet, MAX_LEN);
    let sha256_hash = sha256(&worst_case_pw);
    let sha256_hash_str = sha256_hash_to_hex_string(&sha256_hash);
    let sha256_hashing = sha256_hashing(sha256_hash_str.as_str());

    println!(
        "Start benchmark with {} possible combinations",
        combinations_count(&alphabet, MAX_LEN, 0)
    );
    println!("PLEASE MAKE SURE THAT YOU RUN THIS BIN IN RELEASE MODE..OTHERWISE IT TAKES AGES :)");
    println!(
        "Trying to crack '{}'. SHA256 is '{:?}'",
        worst_case_pw, sha256_hash
    );

    // the actual cracking
    let cp = CrackParameter::new(
        sha256_hashing,
        alphabet,
        MAX_LEN,
        0,
        // Without fair mode the benchmark will make the main OS mainly unusable.
        // I experienced this in Windows. To get a sane benchmark result you
        // should deactivate fair mode of course.
        true,
    );
    let result = libbruteforce::crack(cp);

    assert!(
        result.is_success(),
        "A solution MUST be found! Should be '{}'",
        worst_case_pw
    );
    assert!(
        worst_case_pw.eq(result.solution().as_ref().unwrap()),
        "The solution MUST be correct!"
    );

    // output stats
    println!(
        "Found worst case solution for given alphabet with max len = {}",
        MAX_LEN
    );
    println!("Solution is: '{}'", result.solution().as_ref().unwrap());
    println!(
        "Did {} iterations in {} threads in {:>7.3}s",
        result.combinations_total(),
        result.thread_count(),
        result.seconds_as_fraction()
    );
    let iterations_ps = result.combinations_total() as f64 / result.seconds_as_fraction();
    let iterations_ps_pt = iterations_ps / result.thread_count() as f64;
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

fn sha256(input: &str) -> Sha256Hash {
    use sha2::Digest;
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize()
}

fn sha256_hash_to_hex_string(hash: &Sha256Hash) -> String {
    let mut buf = [0; 64];
    hex::encode_to_slice(hash, &mut buf).unwrap();
    String::from_utf8_lossy(&buf).to_string()
}
