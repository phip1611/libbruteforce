//! This module contains utility functions for internal testing and benchmarking.
//! Unit util in general will be placed in the files where the tested functions
//! are located.
//!
//! This is meant to generate common parameter objects to test or benchmark the lib.
//!
//! This module will also contain benchmarking utility functions.

use crate::crack::parameter::CrackParameter;
use crate::symbols::full_alphabet;
use crate::transform_fns::{NO_HASHING, SHA256_HASHING};

/// Creates CrackParameter for full alphabet with identity hashing.
pub fn create_test_crack_params_full_alphabet(target: &String) -> CrackParameter {
    let alphabet = full_alphabet();
    let max_len = target.len() as u32;
    let min_len = 0;
    CrackParameter::new(
        target.clone(),
        alphabet,
        max_len,
        min_len,
        NO_HASHING,
        false
    )
}

/// Creates CrackParameter for full alphabet with sha256 hashing.
pub fn create_test_crack_params_full_alphabet_sha256(target: &String) -> CrackParameter {
    let alphabet = full_alphabet();
    let max_len = 6;
    let min_len = 0;
    CrackParameter::new(
        target.clone(),
        alphabet,
        max_len,
        min_len,
        SHA256_HASHING,
        false
    )
}

/// Creates CrackParameter for full alphabet with sha256 hashing and fair mode.
pub fn create_test_crack_params_full_alphabet_sha256_fair(target: &String) -> CrackParameter {
    let alphabet = full_alphabet();
    let max_len = 5;
    let min_len = 0;
    CrackParameter::new(
        target.clone(),
        alphabet,
        max_len,
        min_len,
        SHA256_HASHING,
        true
    )
}
