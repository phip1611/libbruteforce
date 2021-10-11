//! All functions related for the multithreaded cracking process.
//! The actual cracking happens here.

use log::{info, trace};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use crate::crack::indices::{indices_create, indices_increment_by, indices_to_string};
use crate::crack::parameter::InternalCrackParameter;

/// Spawns all worker threads.
pub fn spawn_worker_threads<T: 'static + Eq + Send + Sync>(
    cp: Arc<InternalCrackParameter<T>>,
    done: Arc<AtomicBool>,
) -> Vec<JoinHandle<Option<String>>> {
    let mut handles = vec![];
    // spawn thread for each cpu
    for tid in 0..cp.thread_count {
        let mut indices = indices_create(cp.max_length, cp.min_length);
        // prepare array for thread with right starting index
        indices_increment_by(&cp.alphabet, &mut indices, tid).expect("Increment failed");
        handles.push(spawn_worker_thread(cp.clone(), done.clone(), indices, tid));
    }
    handles
}

/// Spawns a worker thread with its work loop.
fn spawn_worker_thread<T: 'static + Eq + Send + Sync>(
    cp: Arc<InternalCrackParameter<T>>,
    done: Arc<AtomicBool>,
    indices: Box<[isize]>,
    tid: usize,
) -> JoinHandle<Option<String>> {
    // mark var as mutable for compiler
    let mut indices = indices;

    // Counter for total iterations/total checked values
    let mut iteration_count = 0_usize;

    thread::spawn(move || {
        // The result that the thread calculated/found
        let mut result = None;

        /// The number after how many iterations the thread looks if another thread
        /// is already done, so that we can stop further work. We do this only after
        /// a few millions iterations to keep the overhead low. Tests on my machine
        /// (i5-10600K) showed that 2 million iterations take about 1s - this should be okay
        /// because the overhead is not that big. A test already showed that
        /// increasing this has no real impact on the iterations per s.
        const INTERRUPT_COUNT_THRESHOLD: usize = 1_000_000;
        let mut interrupt_count = INTERRUPT_COUNT_THRESHOLD;

        // infinite incrementing; break inside loop if its the right time for
        loop {
            if interrupt_count == 0 {
                interrupt_count = INTERRUPT_COUNT_THRESHOLD;
                if done.load(Ordering::Relaxed) {
                    trace!("Thread {:>2} stops at {:>6.2}% progress because another thread found a solution", tid, get_percent(&cp, iteration_count));
                    break;
                } else {
                    trace!(
                        "Thread {:>2} is at {:>6.2}% progress",
                        tid,
                        get_percent(&cp, iteration_count)
                    );
                }
            }
            interrupt_count -= 1;

            let res = indices_increment_by(&cp.alphabet, &mut indices, cp.thread_count);
            if res.is_err() {
                info!(
                    "Thread {:>2} checked all possible values without finding a solution. Done.",
                    tid
                );
                break;
            }

            iteration_count += 1;

            let string = indices_to_string(&cp.alphabet, &indices);
            // transform; e.g. hashing
            // extra parentheses to prevent "field, not a method" error
            let transformed_string = (cp.transform_fn)(&string);
            if transformed_string.eq(&cp.target) {
                info!(
                    "Thread {:>2} found a solution at a progress of {:>6.2}%!",
                    tid,
                    get_percent(&cp, iteration_count)
                );
                // let other threads know we are done
                done.store(true, Ordering::Relaxed);
                result = Some(string);
                break;
            }
        }
        result
    })
}

fn get_percent<T: 'static + Eq + Send + Sync>(
    cp: &Arc<InternalCrackParameter<T>>,
    iteration_count: usize,
) -> f64 {
    let total = cp.combinations_p_t as f64;
    let current = iteration_count as f64;
    current / total * 100_f64
}
