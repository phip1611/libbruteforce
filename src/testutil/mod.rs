//! This module contains utility functions for internal testing and benchmarking.
//! Unit util in general will be placed in the files where the tested functions
//! are located.
//!
//! This is meant to generate common parameter objects to test or benchmark the lib.
//!
//! This module will also contain benchmarking utility functions.

use crate::hash_fncs::{no_hashing, sha256_hashing, Sha256Hash};
use crate::symbols::Builder;
use crate::{BasicCrackParameter, CrackParameter, TargetHashInput};

/// Creates CrackParameter for full alphabet with identity hashing.
pub fn create_test_crack_params_full_alphabet(target: &str) -> CrackParameter<String> {
    let alphabet = Builder::new().full().build();
    let max_len = target.len() as u32;
    let min_len = 0;
    CrackParameter::new(
        BasicCrackParameter::new(alphabet, max_len, min_len, false),
        no_hashing(TargetHashInput::HashAsStr(target)),
    )
}

/// Creates CrackParameter for full alphabet with sha256 hashing.
pub fn create_test_crack_params_full_alphabet_sha256(target: &str) -> CrackParameter<Sha256Hash> {
    let alphabet = Builder::new().full().build();
    let max_len = 6;
    let min_len = 0;
    CrackParameter::new(
        BasicCrackParameter::new(alphabet, max_len, min_len, false),
        sha256_hashing(TargetHashInput::HashAsStr(target)),
    )
}

/// Creates CrackParameter for full alphabet with sha256 hashing and fair mode.
pub fn create_test_crack_params_full_alphabet_sha256_fair(
    target: &str,
) -> CrackParameter<Sha256Hash> {
    let alphabet = Builder::new().full().build();
    let max_len = 5;
    let min_len = 0;
    CrackParameter::new(
        BasicCrackParameter::new(alphabet, max_len, min_len, true),
        sha256_hashing(TargetHashInput::HashAsStr(target)),
    )
}
