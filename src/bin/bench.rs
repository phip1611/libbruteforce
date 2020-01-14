//! Starts a cracking process that does the maximum number of iterations for
//! a alphabet and a specified length. Can be used to compare systems and to
//! check the effect of code changes/improvements.

use libbruteforce::symbols::{full_alphabet, combinations_count};
use libbruteforce::CrackParameter;
use libbruteforce::transform_fns::SHA256_HASHING;

fn main() {
    let alphabet = full_alphabet();
    const MAX_LEN: u32 = 4;
    let mut target = String::new();
    let max_index = alphabet.len() - 1;
    for _ in 0..MAX_LEN {
        target.push(alphabet[max_index]);
    }
    let target_hash = SHA256_HASHING(&target);
    println!("Start benchmark with {} possible combinations", combinations_count(&alphabet, MAX_LEN, 0));
    let cp = CrackParameter::new(
        target_hash.clone(), alphabet, MAX_LEN, 0, SHA256_HASHING, false
    );
    let result = libbruteforce::crack(cp);
    assert!(result.is_success(), format!("A solution MUST be found! Should be '{}'", target));
    assert!(target.eq(result.solution.as_ref().unwrap()), "The solution MUST be correct!");
    println!("Did {} iterations in {} threads in {}s", result.combinations_total, result.thread_count, result.seconds_as_fraction);
    let iterations_p_s = result.combinations_total as f64 / result.seconds_as_fraction;
    let iterations_ps_pt = iterations_p_s / result.thread_count as f64;
    println!("{} iterations / s ", iterations_p_s as usize);
    println!("{} iterations / s (per thread)", iterations_ps_pt as usize);
}
