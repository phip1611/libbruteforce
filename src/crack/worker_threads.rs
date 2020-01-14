//! All functions related for the multithreaded cracking process.
//! The actual cracking happens here.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

use crate::crack::indices::{indices_create, indices_increment_by, indices_to_string};
use crate::crack::parameter::InternalCrackParameter;

/// Spawns all worker threads.
pub fn spawn_worker_threads(cp: Arc<InternalCrackParameter>,
                            done: Arc<AtomicBool>) -> Vec<JoinHandle<Option<String>>> {
    let mut handles = vec![];
    // spawn thread for each cpu
    for tid in 0..cp.thread_count {
        let mut indices = indices_create(cp.max_length);
        // prepare array for thread with right starting index
        indices_increment_by(&cp.alphabet, &mut indices, tid).expect("Increment failed");
        handles.push(
            spawn_worker_thread(
                cp.clone(),
                done.clone(),
                indices,
                tid)
        );
    }
    handles
}

/// Spawns a worker thread with its work loop.
fn spawn_worker_thread(cp: Arc<InternalCrackParameter>,
                       done: Arc<AtomicBool>,
                       indices: Box<[isize]>,
                       _tid: usize) -> JoinHandle<Option<String>> {
    // mark var as mutable for compiler
    let mut indices = indices;
    thread::spawn(move || {
        // The result that the thread calculated/found
        let mut result = None;

        /// The number after how many iterations the thread looks if another thread
        /// is already done, so that we can stop further work. We do this only after
        /// a few millions iterations to keep the overhead low. Tests on my machine
        /// showed that 1 million iterations take about 1.6s - this should be okay
        /// because the overhead is not that big
        const INTERRUPT_COUNT_THRESHOLD: usize = 1_000_000;
        let mut interrupt_count = INTERRUPT_COUNT_THRESHOLD;

        // infinite incrementing; break inside loop if its the right time for
        loop {
            interrupt_count -= 1;
            if interrupt_count == 0 {
                interrupt_count = INTERRUPT_COUNT_THRESHOLD;
                if done.load(Ordering::SeqCst) { break; }
            }

            let res = indices_increment_by(&cp.alphabet, &mut indices, cp.thread_count);
            if res.is_err() { break; }

            let string = indices_to_string(&cp.alphabet, &indices);
            // transform; e.g. hashing
            // extra parantheses to prevent "field, not a method" error
            let transformed_string = (cp.transform_fn)(&string);
            if transformed_string.eq(&cp.target) {
                // let other threads know we are done
                done.store(true, Ordering::SeqCst);
                result = Some(string);
            }
        }
        result
    })
}
