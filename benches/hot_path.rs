/*
MIT License

Copyright (c) 2026 Philipp Schuster

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
//! Criterion benchmarks for the cracking hot path.
//!
//! The innermost worker loop repeats three steps for every candidate: advance
//! the indices to the next candidate, render them into a string, and hash that
//! string.
//!
//! * `candidate_generation` measures the first two steps single-threaded and in
//!   isolation, so it is fast and low-noise and reliably shows the effect of
//!   changes to the candidate-generation code.
//! * `hashing` is included for context: it is the dominant per-candidate cost in
//!   a real run, so the generation numbers should be weighed against it.
//! * `end_to_end` runs the full multi-threaded `crack()` over a small,
//!   deterministic search space. It is the only benchmark that exercises the
//!   worker loop itself (thread orchestration, per-iteration overhead), but it
//!   is noisier, so it suits coarse A/B comparisons rather than sub-percent
//!   claims.

use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use libbruteforce::bench_internals::{indices_create, indices_increment_by, indices_to_string};
use libbruteforce::hash_fncs::{no_hashing, sha256_hashing};
use libbruteforce::symbols::{Builder, combination_count};
use libbruteforce::{BasicCrackParameter, CrackParameter, TargetHashInput, crack};
use std::hint::black_box;

/// Candidate length used by the generation benchmarks.
///
/// Combined with the 62-symbol alphabet below the search space (62^8) is far
/// larger than any benchmark iteration count, so repeated incrementing never
/// overflows during a run.
const LEN: u32 = 8;

fn alphabet() -> Box<[char]> {
    Builder::new().with_letters().with_digits().build()
}

fn bench_candidate_generation(c: &mut Criterion) {
    let alphabet = alphabet();

    let mut group = c.benchmark_group("candidate_generation");
    group.throughput(Throughput::Elements(1));

    // Pure incrementing: advance the indices to the next candidate.
    group.bench_function("increment", |b| {
        let mut indices = indices_create(LEN, 0);
        b.iter(|| {
            indices_increment_by(&alphabet, &mut indices, 1)
                .expect("length 8 has enough headroom for the benchmark");
            black_box(&indices);
        });
    });

    // Pure rendering: turn a fully populated (max length) indices array into a
    // string. Rendering the longest candidate is the worst case.
    group.bench_function("to_string", |b| {
        let mut indices = indices_create(LEN, 0);
        let mid = (alphabet.len() / 2) as isize;
        indices.iter_mut().for_each(|slot| *slot = mid);
        let mut buf = String::with_capacity(LEN as usize * 4);
        b.iter(|| {
            indices_to_string(&mut buf, &alphabet, &indices);
            black_box(&buf);
        });
    });

    // Combined: the real per-candidate cost, excluding hashing.
    group.bench_function("increment_and_to_string", |b| {
        let mut indices = indices_create(LEN, 0);
        let mut buf = String::with_capacity(LEN as usize * 4);
        b.iter(|| {
            indices_increment_by(&alphabet, &mut indices, 1)
                .expect("length 8 has enough headroom for the benchmark");
            indices_to_string(&mut buf, &alphabet, &indices);
            black_box(&buf);
        });
    });

    group.finish();
}

fn bench_hashing(c: &mut Criterion) {
    // The dominant per-candidate cost in a real run, shown for context.
    let hasher = sha256_hashing(TargetHashInput::Plaintext("benchmark"));

    let mut group = c.benchmark_group("hashing");
    group.throughput(Throughput::Elements(1));
    group.bench_function("sha256_short_input", |b| {
        b.iter(|| black_box(hasher.hash(black_box("abcdef"))));
    });
    group.finish();
}

/// Alphabet and length for the end-to-end benchmarks. Digits (10 symbols) with
/// length 6 give ~1.1M candidates: large enough that the actual cracking work
/// dominates the thread spawn/join overhead of a single `crack()` call, yet
/// small enough to finish in a few milliseconds.
const E2E_MAX_LEN: u32 = 6;

fn digits() -> Box<[char]> {
    Builder::new().with_digits().build()
}

/// Worst-case target for a full search: the last candidate in the space (the
/// highest symbol repeated to the maximum length). Every candidate is checked
/// before it is found, so the measured work is deterministic.
fn worst_case_password(alphabet: &[char], len: u32) -> String {
    let last = *alphabet.last().expect("alphabet must not be empty");
    std::iter::repeat_n(last, len as usize).collect()
}

fn bench_end_to_end(c: &mut Criterion) {
    let alphabet = digits();
    let worst_case = worst_case_password(&alphabet, E2E_MAX_LEN);
    let total = combination_count(&alphabet, E2E_MAX_LEN, 0);

    let mut group = c.benchmark_group("end_to_end");
    group.throughput(Throughput::Elements(total as u64));
    // Each iteration spawns and joins the full worker-thread pool, so keep the
    // sample count modest to bound the total runtime. Multi-threaded runs are
    // noisier than the single-threaded micro-benchmarks above; use them for
    // coarse A/B comparisons, not for sub-percent claims.
    group.sample_size(30);

    // Generation-dominated: no hashing, so per-candidate work is small and
    // worker-loop overhead is most visible.
    group.bench_function("crack_no_hashing", |b| {
        b.iter_batched(
            || {
                CrackParameter::new(
                    BasicCrackParameter::new(alphabet.clone(), E2E_MAX_LEN, 0, false),
                    no_hashing(TargetHashInput::Plaintext(&worst_case)),
                )
            },
            crack,
            BatchSize::PerIteration,
        );
    });

    // Realistic: sha256 hashing dominates the per-candidate cost.
    group.bench_function("crack_sha256", |b| {
        b.iter_batched(
            || {
                CrackParameter::new(
                    BasicCrackParameter::new(alphabet.clone(), E2E_MAX_LEN, 0, false),
                    sha256_hashing(TargetHashInput::Plaintext(&worst_case)),
                )
            },
            crack,
            BatchSize::PerIteration,
        );
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_candidate_generation,
    bench_hashing,
    bench_end_to_end
);
criterion_main!(benches);
