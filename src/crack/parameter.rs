//! Describes the structs with all the necessary parameters for the
//! whole multithreaded cracking process.

use crate::symbols::combinations_count;
use crate::{CrackTarget, TargetHashAndHashFunction};
use std::fmt::Debug;

/// Crack parameter for [`crack`]. It combines the basic struct [`BasicCrackParameter`]
/// with the generic [`TargetHashAndHashFunction`]. This separation exists so that
/// hash selection functions can be written more convenient.
///
/// # Example
/// ```ignore
/// use libbruteforce::{BasicCrackParameter, CrackParameter, TargetHashInput};
/// use libbruteforce::hash_fncs::sha256_hashing;
///
/// // sha256("a+c")
/// let sha256_hash = "3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d";
///
/// let res = CrackParameter::new(
///         BasicCrackParameter::new(alphabet, max_len, min_len, false),
///         sha256_hashing(TargetHashInput::HashAsStr(sha256_hash)),
/// );
/// ```
#[derive(Debug)]
pub struct CrackParameter<T: CrackTarget> {
    /// Basic parameters.
    basic: BasicCrackParameter,
    /// Target hash and hashing algorithm.
    target_hash_and_hash_fnc: TargetHashAndHashFunction<T>,
}

impl<T: CrackTarget> CrackParameter<T> {
    /// Constructor.
    ///
    /// # Example
    /// ```ignore
    /// use libbruteforce::{BasicCrackParameter, CrackParameter, TargetHashInput};
    /// use libbruteforce::hash_fncs::sha256_hashing;
    ///
    /// // sha256("a+c")
    /// let sha256_hash = "3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d";
    ///
    /// let res = CrackParameter::new(
    ///         BasicCrackParameter::new(alphabet, max_len, min_len, false),
    ///         sha256_hashing(TargetHashInput::HashAsStr(sha256_hash)),
    /// );
    /// ```
    pub fn new(basic: BasicCrackParameter, crack_info: TargetHashAndHashFunction<T>) -> Self {
        Self {
            basic,
            target_hash_and_hash_fnc: crack_info,
        }
    }

    /// Convenient wrapper for [`BasicCrackParameter::alphabet`].
    pub fn alphabet(&self) -> &[char] {
        self.basic.alphabet()
    }

    /// Convenient wrapper for [`BasicCrackParameter::max_length`].
    pub fn max_length(&self) -> u32 {
        self.basic.max_length()
    }

    /// Convenient wrapper for [`BasicCrackParameter::min_length`].
    pub fn min_length(&self) -> u32 {
        self.basic.min_length()
    }

    /// Convenient wrapper for [`BasicCrackParameter::fair_mode`].
    pub fn fair_mode(&self) -> bool {
        self.basic.fair_mode()
    }

    pub fn target_hash_and_hash_fnc(&self) -> &TargetHashAndHashFunction<T> {
        &self.target_hash_and_hash_fnc
    }
}

/// Describes the necessary parameters for the [`crate::crack`]-function
/// without the generic part that is outsourced to [`crate::TargetHashAndHashFunction`].
#[derive(Debug)]
pub struct BasicCrackParameter {
    /// all symbols (letters, digits, ...)
    alphabet: Box<[char]>,
    /// maximum crack length (to limit possible combinations)
    max_length: u32,
    /// minimum crack length (to limit possible combinations)
    min_length: u32,
    /// use n-1 threads to save system resources
    fair_mode: bool,
}

impl BasicCrackParameter {
    /// Constructor.
    pub fn new(alphabet: Box<[char]>, max_length: u32, min_length: u32, fair_mode: bool) -> Self {
        Self {
            alphabet,
            max_length,
            min_length,
            fair_mode,
        }
    }

    pub fn alphabet(&self) -> &[char] {
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

/// Internal wrapper around [`CrackParameter`], that holds additional
/// information for the multi-threaded cracking process.
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
    /// Convenient shortcut around [`crate::TargetHashAndHashFunction::hash_matches`].
    pub fn hash_matches(&self, input: &str) -> bool {
        self.crack_param()
            .target_hash_and_hash_fnc
            .hash_matches(input)
    }
}

impl<T: CrackTarget> From<CrackParameter<T>> for InternalCrackParameter<T> {
    /// Creates the object used internally for the cracking process from
    /// what the user/programmer has given the lib through the public api.
    fn from(cp: CrackParameter<T>) -> Self {
        let combinations_total =
            combinations_count(&cp.basic.alphabet, cp.basic.max_length, cp.basic.min_length);

        let mut thread_count = get_thread_count(cp.basic.fair_mode);
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
