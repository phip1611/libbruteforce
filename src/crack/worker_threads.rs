//! All functions related for the multithreaded cracking process.
//! The actual cracking happens here.

use log::{info, trace};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use crate::crack::indices::{indices_create, indices_increment_by, indices_to_string};
use crate::crack::parameter::InternalCrackParameter;
use crate::CrackTarget;

/// Spawns all worker threads.
pub fn spawn_worker_threads<T: CrackTarget>(
    params: Arc<InternalCrackParameter<T>>,
    done: Arc<AtomicBool>,
) -> Vec<JoinHandle<Option<String>>> {
    let mut handles = vec![];
    // spawn thread for each cpu
    for tid in 0..params.thread_count {
        // indices object, that each thread gets as starting point
        let mut indices =
            indices_create(params.crack_param.max_length, params.crack_param.min_length);

        // alternate indices object for the next thread
        indices_increment_by(&params.crack_param.alphabet, &mut indices, tid)
            .expect("Increment failed");

        handles.push(spawn_worker_thread(
            params.clone(),
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
    done: Arc<AtomicBool>,
    indices: Box<[isize]>,
    tid: usize,
) -> JoinHandle<Option<String>> {
    // mark var as mutable for compiler
    let mut indices = indices;

    // Counter for total iterations/total checked values
    let mut iteration_count = 0_usize;

    thread::spawn(move || {
        // reserve a string buffer with the maximum needed size; in the worst case it can contain
        // indices.len() * 4 bytes, because UTF-8 chars can be at most 4 byte long. Because
        // I prevent the allocation for a string in every iteration and do this only once,
        // I cauld improve the performance even further.
        let mut current_crack_string = String::with_capacity(indices.len() * 4);

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

            {
                let res = indices_increment_by(
                    &params.crack_param.alphabet,
                    &mut indices,
                    params.thread_count,
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
                    &params.crack_param.alphabet,
                    &indices,
                );

                // transform; e.g. hashing
                // extra parentheses to prevent "field, not a method" error
                let transformed_string = (params.crack_param.transform_fn)(&current_crack_string);
                if transformed_string.eq(&params.crack_param.target) {
                    info!(
                        "Thread {:>2} found a solution at a progress of {:>6.2}%!",
                        tid,
                        get_percent(&params, iteration_count)
                    );
                    // let other threads know we are done
                    done.store(true, Ordering::Relaxed);
                    result = Some(current_crack_string);
                    break;
                }
            }
        }
        result
    })
}

fn get_percent<T: CrackTarget>(cp: &Arc<InternalCrackParameter<T>>, iteration_count: usize) -> f64 {
    let total = cp.combinations_p_t as f64;
    let current = iteration_count as f64;
    current / total * 100.0
}
