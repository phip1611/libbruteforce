//! Describes the result of the cracking process.

use crate::crack::parameter::InternalCrackParameter;

/// Describes the result of a finished cracking process.
pub struct CrackResult {
    /// the target string
    pub target: String,
    /// The solution to the target string
    pub solution: Option<String>,
    pub thread_count: usize,
    /// Total combinations (from length and alphabet). Note that
    /// this is in almost any case much higher than actual combinations
    /// were needed to test.
    pub combinations_total: usize,
    /// Combinations each thread had to to (in worst case)
    pub combinations_p_t: usize,
    /// Duration until the solution has been found.
    pub seconds_as_fraction: f64,
}

impl CrackResult {
    fn new(
        cp: InternalCrackParameter,
        seconds_as_fraction: f64,
        solution: Option<String>,
    ) -> CrackResult {
        CrackResult {
            target: cp.target,
            solution,
            thread_count: cp.thread_count,
            combinations_total: cp.combinations_total,
            combinations_p_t: cp.combinations_p_t,
            seconds_as_fraction,
        }
    }

    pub fn failure(cp: InternalCrackParameter, seconds_as_fraction: f64) -> CrackResult {
        CrackResult::new(cp, seconds_as_fraction, None)
    }

    pub fn success(
        cp: InternalCrackParameter,
        seconds_as_fraction: f64,
        solution: String,
    ) -> CrackResult {
        CrackResult::new(cp, seconds_as_fraction, Some(solution))
    }

    pub fn is_failure(&self) -> bool {
        self.solution.is_none()
    }

    pub fn is_success(&self) -> bool {
        self.solution.is_some()
    }
}
