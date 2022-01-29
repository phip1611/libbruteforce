//! Describes the structs with all the necessary parameters for the
//! whole multithreaded cracking process.

use crate::symbols::combinations_count;
use crate::{CrackTarget, TargetHashAndHashFunction, TargetHashAndHashFunctionTrait};
use std::fmt::{Debug, Formatter};

/// Describes the necessary parameters for the [`crate::crack`]-function.
pub struct CrackParameter<T: CrackTarget> {
    /// Target hash and hashing algorithm.
    crack_info: TargetHashAndHashFunction<T>,
    /// all symbols (letters, digits, ...)
    alphabet: Box<[char]>,
    /// maximum crack length (to limit possible combinations)
    max_length: u32,
    /// minimum crack length (to limit possible combinations)
    min_length: u32,
    /// use n-1 threads to save system resources
    fair_mode: bool,
}

impl<T: CrackTarget> CrackParameter<T> {
    /// Constructor.
    pub fn new(
        crack_info: TargetHashAndHashFunction<T>,
        alphabet: Box<[char]>,
        max_length: u32,
        min_length: u32,
        fair_mode: bool,
    ) -> Self {
        Self {
            crack_info,
            alphabet,
            max_length,
            min_length,
            fair_mode,
        }
    }

    pub fn crack_info(&self) -> &TargetHashAndHashFunction<T> {
        &self.crack_info
    }
    pub fn alphabet(&self) -> &Box<[char]> {
        &self.alphabet
    }
    pub fn max_length(&self) -> u32 {
        self.max_length
    }
    pub fn min_length(&self) -> u32 {
        self.min_length
    }
    pub fn fair_mode(&self) -> bool {
        self.fair_mode
    }
}

impl<T: CrackTarget> Debug for CrackParameter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CrackParameter")
            .field("crack_info", &self.crack_info)
            .field("alphabet", &self.alphabet)
            .field("max_length", &self.max_length)
            .field("min_length", &self.min_length)
            .field("fair_mode", &self.fair_mode)
            .finish()
    }
}

/// Internal wrapper around [`CrackParameter`], that holds important information for
/// the cracking process.
#[derive(Debug)]
pub(crate) struct InternalCrackParameter<T: CrackTarget> {
    /// See [`CrackParameter`].
    crack_param: CrackParameter<T>,
    /// thread count
    thread_count: usize,
    /// total combinations (given by alphabet and length)
    combinations_total: usize,
    /// Max possible combinations per thread (```combinations_total / thread_count```).
    /// If ```combinations_total % thread_count != 0``` then this value will
    /// be rounded down. This is only for informational use, the internal algorithm will
    /// still consider all possible combinations.
    ///
    /// A thread reaches this number only in worst case.
    combinations_p_t: usize,
}

impl<T: CrackTarget> InternalCrackParameter<T> {
    pub fn crack_param(&self) -> &CrackParameter<T> {
        &self.crack_param
    }
    pub fn thread_count(&self) -> usize {
        self.thread_count
    }
    pub fn combinations_total(&self) -> usize {
        self.combinations_total
    }
    pub fn combinations_p_t(&self) -> usize {
        self.combinations_p_t
    }
    /// Convenient shortcut around [`crate::TargetHashAndHashFunctionTrait::hash_matches`].
    pub fn hash_matches(&self, input: &str) -> bool {
        self.crack_param().crack_info().hash_matches(input)
    }
}

impl<T: CrackTarget> From<CrackParameter<T>> for InternalCrackParameter<T> {
    /// Creates the object used internally for the cracking process from
    /// what the user/programmer has given the lib through the public api.
    fn from(cp: CrackParameter<T>) -> Self {
        let combinations_total = combinations_count(&cp.alphabet, cp.max_length, cp.min_length);

        let mut thread_count = get_thread_count(cp.fair_mode);
        if thread_count > combinations_total {
            // this only scales because I have the assumption, that there will never be thousands
            // of CPU threads/cores.
            log::trace!("there are so few combinations to check, that only one thread is used");
            thread_count = 1;
        }

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
