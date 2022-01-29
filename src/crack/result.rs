//! Describes the result of the cracking process.

use crate::crack::parameter::InternalCrackParameter;
use crate::{CrackTarget, TargetHashAndHashFunctionTrait};

/// Describes the result of a finished cracking process.
#[derive(Debug)]
pub struct CrackResult<T: CrackTarget> {
    /// the target string
    target: T,
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

impl<T: CrackTarget> CrackResult<T> {
    fn new(
        cp: InternalCrackParameter<T>,
        seconds_as_fraction: f64,
        solution: Option<String>,
    ) -> Self {
        Self {
            target: cp.crack_param().crack_info().get_target().clone(),
            solution,
            thread_count: cp.thread_count(),
            combinations_total: cp.combinations_total(),
            combinations_p_t: cp.combinations_p_t(),
            seconds_as_fraction,
        }
    }

    pub(crate) fn new_failure(cp: InternalCrackParameter<T>, seconds_as_fraction: f64) -> Self {
        Self::new(cp, seconds_as_fraction, None)
    }

    pub(crate) fn new_success(
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


    pub fn target(&self) -> &T {
        &self.target
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
}
