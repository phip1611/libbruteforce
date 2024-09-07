/*
MIT License

Copyright (c) 2022 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
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
    duration_in_seconds: f64,
}

impl CrackResult {
    fn new<T: CrackTarget>(
        cp: &InternalCrackParameter<T>,
        duration_in_seconds: f64,
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
            duration_in_seconds,
        }
    }

    pub(crate) fn new_failure<T: CrackTarget>(
        cp: &InternalCrackParameter<T>,
        seconds_as_fraction: f64,
    ) -> Self {
        Self::new(cp, seconds_as_fraction, None)
    }

    pub(crate) fn new_success<T: CrackTarget>(
        cp: &InternalCrackParameter<T>,
        seconds_as_fraction: f64,
        solution: String,
    ) -> Self {
        Self::new(cp, seconds_as_fraction, Some(solution))
    }

    /// Returns true, if no solution was found.
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        self.solution.is_none()
    }

    /// Returns true, if a solution was found.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.solution.is_some()
    }

    /// Returns the solution, if any.
    #[must_use]
    pub fn solution(&self) -> Option<&str> {
        self.solution.as_deref()
    }

    /// Returns the number of threads that were used.
    #[must_use]
    pub const fn thread_count(&self) -> usize {
        self.thread_count
    }

    /// Returns the number of total combinations that would have been possible (worst case).
    #[must_use]
    pub const fn combinations_total(&self) -> usize {
        self.combinations_total
    }

    /// Returns the number of combinations each thread has to check in the worst case.
    #[must_use]
    pub const fn combinations_p_t(&self) -> usize {
        self.combinations_p_t
    }

    /// Returns the duration of the cracking process in seconds.
    #[must_use]
    pub const fn duration_in_seconds(&self) -> f64 {
        self.duration_in_seconds
    }

    /// Returns the target hash that needed to be cracked.
    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }
}
