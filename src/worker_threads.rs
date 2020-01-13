use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use crate::transformation_fns::TransformationFn;
use crate::indices::{indices_create, indices_increment_by, indices_to_string};
use std::thread;

/// Spawns all worker threads.
pub fn spawn_worker_threads(done: Arc<AtomicBool>,
                        target: Arc<String>,
                        transform_fn: TransformationFn,
                        alphabet: Arc<Box<[char]>>,
                        thread_count: usize,
                        max_length: usize) -> Vec<JoinHandle<Option<String>>> {
    let mut handles = vec![];
    for tid in 0..thread_count {
        // spawn thread for each cpu
        let mut indices = indices_create(max_length);

        // variables needed in thread
        let target = Arc::clone(&target);
        let alphabet = Arc::clone(&alphabet);

        let done = done.clone();

        // prepare array for thread with right starting index
        indices_increment_by(&alphabet, &mut indices, tid).expect("Increment failed");

        handles.push(
            spawn_worker_thread(
                done,
                target,
                transform_fn,
                indices,
                alphabet,
                thread_count,
            )
        );
    }
    handles
}

/// Spawns a worker thread with its work loop.
fn spawn_worker_thread(done: Arc<AtomicBool>,
                       target: Arc<String>,
                       transform_fn: TransformationFn,
                       indices: Box<[isize]>,
                       alphabet: Arc<Box<[char]>>,
                       thread_count: usize) -> JoinHandle<Option<String>> {
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

            let res = indices_increment_by(&alphabet, &mut indices, thread_count);
            if res.is_err() { break; }

            let string = indices_to_string(&alphabet, &indices);
            // transform; e.g. hashing
            let transformed_string = transform_fn(&string);
            if transformed_string.eq(target.as_ref()) {
                // let other threads know we are done
                done.store(true, Ordering::SeqCst);
                result = Some(string);
            }
        }
        result
    })
}