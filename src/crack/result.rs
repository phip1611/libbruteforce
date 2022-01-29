//! Describes the result of the cracking process.

use crate::{CrackTarget, InternalCrackParameter};

/// Describes the result of a finished cracking process.
#[derive(Debug)]
pub struct CrackResult {
    /// The target hash string representation.
    target: String,
    /// The solution to the target string
    solution: Option<String>,
    /// Amount of threads to use.
    thread_count: usize,
    /// Total combinations (from length and alphabet). Note that
    /// this is in almost any case much higher than actual combinations
    /// were needed to test.
    combinations_total: usize,
    /// Combinations each thread had to to (in worst case)
    combinations_p_t: usize,
    /// Duration until the solution has been found.
    seconds_as_fraction: f64,
}

impl CrackResult {
    fn new<T: CrackTarget>(
        cp: InternalCrackParameter<T>,
        seconds_as_fraction: f64,
        solution: Option<String>,
    ) -> Self {
        let th = cp.crack_param().target_hash_and_hash_fnc();
        let target_hash_as_str = th.hash_type_to_str_repr(th.target_hash());
        Self {
            target: target_hash_as_str,
            solution,
            thread_count: cp.thread_count(),
            combinations_total: cp.combinations_total(),
            combinations_p_t: cp.combinations_p_t(),
            seconds_as_fraction,
        }
    }

    pub(crate) fn new_failure<T: CrackTarget>(
        cp: InternalCrackParameter<T>,
        seconds_as_fraction: f64,
    ) -> Self {
        Self::new(cp, seconds_as_fraction, None)
    }

    pub(crate) fn new_success<T: CrackTarget>(
        cp: InternalCrackParameter<T>,
        seconds_as_fraction: f64,
        solution: String,
    ) -> Self {
        Self::new(cp, seconds_as_fraction, Some(solution))
    }

    pub fn is_failure(&self) -> bool {
        self.solution.is_none()
    }

    pub fn is_success(&self) -> bool {
        self.solution.is_some()
    }

    pub fn solution(&self) -> &Option<String> {
        &self.solution
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
    pub fn seconds_as_fraction(&self) -> f64 {
        self.seconds_as_fraction
    }
    pub fn target(&self) -> &str {
        &self.target
    }
}
