//! Describes the structs with all the necessary parameters for the
//! whole multithreaded cracking process.

use crate::symbols::combinations_count;
use crate::transform_fns::TransformFn;
use crate::CrackTarget;
use std::fmt::{
    Debug,
    Formatter,
};

/// Describes the necessary parameters for the `crack`-function. This is part of
/// the public API.
pub struct CrackParameter<T: CrackTarget> {
    /// hash to crack
    pub target: T,
    /// all symbols (letters, digits, ...)
    pub alphabet: Box<[char]>,
    /// maximum crack length (to limit possible combinations)
    pub max_length: u32,
    /// minimum crack length (to limit possible combinations)
    pub min_length: u32,
    /// hashing function
    pub transform_fn: TransformFn<T>,
    /// use n-1 threads to save system resources
    pub fair_mode: bool,
}

impl<T: CrackTarget> CrackParameter<T> {
    pub fn new(
        target: T,
        alphabet: Box<[char]>,
        max_length: u32,
        min_length: u32,
        transform_fn: TransformFn<T>,
        fair_mode: bool,
    ) -> Self {
        Self {
            target,
            alphabet,
            max_length,
            min_length,
            transform_fn,
            fair_mode,
        }
    }
}

impl<T: CrackTarget> Debug for CrackParameter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CrackParameter")
            .field("target", &self.target)
            .field("alphabet", &self.alphabet)
            .field("max_length", &self.max_length)
            .field("min_length", &self.min_length)
            .field("fair_mode", &self.fair_mode)
            .field("transform_fn", &"<code>")
            .finish()
    }
}

/// Internal wrapper around [`CrackParameter`], that holds important information for
/// the cracking process.
#[derive(Debug)]
pub struct InternalCrackParameter<T: CrackTarget> {
    /// See [`CrackParameter`].
    pub crack_param: CrackParameter<T>,
    /// thread count
    pub thread_count: usize,
    /// total combinations (given by alphabet and length)
    pub combinations_total: usize,
    /// max possible combinations per thread (```combinations_total / thread_count```).
    /// if ```combinations_total % thread_count != 0``` then this value will
    /// be rounded down. This number is only reached in worst case.
    pub combinations_p_t: usize,
}

impl<T: CrackTarget> From<CrackParameter<T>> for InternalCrackParameter<T> {
    /// Creates the object used internally for the cracking process from
    /// what the user/programmer has given the lib through the public api.
    fn from(cp: CrackParameter<T>) -> Self {
        let combinations_total = combinations_count(&cp.alphabet, cp.max_length, cp.min_length);
        let thread_count = get_thread_count(cp.fair_mode);
        let combinations_p_t = combinations_total / thread_count;
        Self {
            crack_param: cp,
            thread_count,
            combinations_total,
            combinations_p_t,
        }
    }
}

/// Returns the thread count for cracking. ```fair_mode```means
/// that n-1 thread is used so that the host system is less likely
/// to hang during the process.
fn get_thread_count(fair_mode: bool) -> usize {
    let cpus = num_cpus::get();
    if cpus > 1 && fair_mode {
        cpus - 1
    } else {
        cpus
    }
}
