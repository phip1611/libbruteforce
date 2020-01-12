//! This module offers some benchmarking functions to check how many combinations your system
//! can do in a specific time.

use std::time::Instant;
use crate::symbols::{build_alphabet, combinations_count};
use crate::crack;
use crate::transformation_fns::NO_HASHING;

pub struct BenchmarkResult {
    combinations_count: usize,
    seconds: f64,
}

pub fn bench() -> BenchmarkResult {
    let alphabet = build_alphabet(
        false,
        false,
        true,
        false,
        false,
        false,
        false
    );
    let target = String::from("9999999");
    let max_len = target.len();
    let count = combinations_count(&alphabet, max_len as u32);
    let instant = Instant::now();
    let res = crack(target.clone(), alphabet, max_len, NO_HASHING, false);
    let time = instant.elapsed().as_micros() / 1000;
    let time = time as f64 / 1000_f64;
    assert!(res.is_some());
    assert!(res.unwrap().eq(&target));
    BenchmarkResult {
        combinations_count: count,
        seconds: time
    }
}

#[cfg(test)]
mod tests {
    use crate::benchmark::bench;

    #[test]
    fn run_bench() {
        let res = bench();
        println!("{} combinations in {}s", res.combinations_count, res.seconds);
    }

}
