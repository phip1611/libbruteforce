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
//! All functions related for the multithreaded cracking process.
//! The actual cracking happens here in the closure in [`spawn_worker_thread`].

use log::{info, trace};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use crate::crack::indices::{indices_create, indices_increment_by, indices_to_string};
use crate::{CrackTarget, InternalCrackParameter};

/// Spawns all worker threads.
pub(crate) fn spawn_worker_threads<T: CrackTarget>(
    params: Arc<InternalCrackParameter<T>>,
    sender: Sender<String>,
    done: Arc<AtomicBool>,
) -> Vec<JoinHandle<()>> {
    let mut handles = vec![];
    // spawn thread for each cpu
    for tid in 0..params.thread_count() {
        // indices object, that each thread gets as starting point
        let mut indices = indices_create(
            params.crack_param().max_length(),
            params.crack_param().min_length(),
        );

        // alternate indices object for the next thread
        indices_increment_by(params.crack_param().alphabet(), &mut indices, tid)
            .expect("Increment failed");

        handles.push(spawn_worker_thread(
            params.clone(),
            sender.clone(),
            done.clone(),
            indices,
            tid,
        ));
    }
    handles
}

/// Spawns a worker thread with its work loop.
fn spawn_worker_thread<T: CrackTarget>(
    params: Arc<InternalCrackParameter<T>>,
    sender: Sender<String>,
    done: Arc<AtomicBool>,
    mut indices: Box<[isize]>,
    tid: usize,
) -> JoinHandle<()> {
    // Counter for total iterations/total checked values
    let mut iteration_count = 0;

    thread::spawn(move || {
        // reserve a string buffer with the maximum needed size; in the worst case it can contain
        // indices.len() * 4 bytes, because UTF-8 chars can be at most 4 byte long. Because
        // I prevent the allocation for a string in every iteration and do this only once,
        // I could improve the performance even further.
        let mut current_crack_string = String::with_capacity(indices.len() * 4);

        /// The amount of iterations after the thread checks if another thread
        /// is already done, so that we can stop further work. We do this only after
        /// a few millions iterations to keep the overhead low. Tests on my machine
        /// (i5-10600K) showed that 2 million iterations take about 1s - this should be okay
        /// because the overhead is not that big. A test already showed that
        /// increasing this has no real impact on the iterations per s.
        const INTERRUPT_COUNT_THRESHOLD: usize = 2_000_000;
        let mut interrupt_count = INTERRUPT_COUNT_THRESHOLD;

        // infinite incrementing; break inside loop if its the right time for
        loop {
            // tell about progress + stop if another thread found a solution
            {
                if interrupt_count == 0 {
                    interrupt_count = INTERRUPT_COUNT_THRESHOLD;
                    if done.load(Ordering::Relaxed) {
                        trace!("Thread {:>2} stops at {:>6.2}% progress because another thread found a solution", tid, get_percent(&params, iteration_count));
                        break;
                    } else {
                        trace!(
                            "Thread {:>2} is at {:>6.2}% progress",
                            tid,
                            get_percent(&params, iteration_count)
                        );
                    }
                }
                interrupt_count -= 1;
            }

            // the actual cracking
            {
                let res = indices_increment_by(
                    params.crack_param().alphabet(),
                    &mut indices,
                    params.thread_count(),
                );
                if res.is_err() {
                    info!(
                        "Thread {:>2} checked all possible values without finding a solution. Done.",
                        tid
                    );
                    break;
                }

                iteration_count += 1;

                // build string
                indices_to_string(
                    &mut current_crack_string,
                    params.crack_param().alphabet(),
                    &indices,
                );

                // transform; e.g. hashing
                // extra parentheses to prevent "field, not a method" error
                if params.hash_matches(current_crack_string.as_str()) {
                    info!(
                        "Thread {:>2} found a solution at a progress of {:>6.2}%!",
                        tid,
                        get_percent(&params, iteration_count)
                    );
                    // send the result to the main thread
                    if sender.send(current_crack_string.clone()).is_err() {
                        // and quit if the channel is closed
                        trace!("Thread {:>2} stops at {:>6.2}% progress because the solutions channel is closed", tid, get_percent(&params, iteration_count));
                        break;
                    }
                }
            }
        }
    })
}

/// Returns the percent of all possible iterations that the current thread has already
/// executed.
#[inline]
fn get_percent<T: CrackTarget>(cp: &InternalCrackParameter<T>, iteration_count: u64) -> f32 {
    let total = cp.combinations_p_t() as f32;
    let current = iteration_count as f32;
    current / total * 100.0
}
