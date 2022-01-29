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
use crate::symbols::combination_count;
use crate::{CrackParameter, CrackTarget};

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
        let combinations_total = combination_count(
            &cp.basic.alphabet(),
            cp.basic.max_length(),
            cp.basic.min_length(),
        );

        let mut thread_count = get_thread_count(cp.basic.fair_mode());
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
