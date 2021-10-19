//! This module contains utility functions for internal testing and benchmarking.
//! Unit util in general will be placed in the files where the tested functions
//! are located.
//!
//! This is meant to generate common parameter objects to test or benchmark the lib.
//!
//! This module will also contain benchmarking utility functions.

use crate::crack::parameter::CrackParameter;
use crate::symbols::Builder;
use crate::transform_fns::{
    str_to_sha256_hash,
    Sha256Hash,
    NO_HASHING,
    SHA256_HASHING,
};

/// Creates CrackParameter for full alphabet with identity hashing.
#[allow(dead_code)]
pub fn create_test_crack_params_full_alphabet(target: &str) -> CrackParameter<String> {
    let alphabet = Builder::new().full().build();
    let max_len = target.len() as u32;
    let min_len = 0;
    CrackParameter::new(
        target.to_owned(),
        alphabet,
        max_len,
        min_len,
        NO_HASHING,
        false,
    )
}

/// Creates CrackParameter for full alphabet with sha256 hashing.
#[allow(dead_code)]
pub fn create_test_crack_params_full_alphabet_sha256(target: &str) -> CrackParameter<Sha256Hash> {
    let target = str_to_sha256_hash(target);
    let alphabet = Builder::new().full().build();
    let max_len = 6;
    let min_len = 0;
    CrackParameter::new(target, alphabet, max_len, min_len, SHA256_HASHING, false)
}

/// Creates CrackParameter for full alphabet with sha256 hashing and fair mode.
#[allow(dead_code)]
pub fn create_test_crack_params_full_alphabet_sha256_fair(
    target: &str,
) -> CrackParameter<Sha256Hash> {
    let target = str_to_sha256_hash(target);
    let alphabet = Builder::new().full().build();
    let max_len = 5;
    let min_len = 0;
    CrackParameter::new(target, alphabet, max_len, min_len, SHA256_HASHING, true)
}
