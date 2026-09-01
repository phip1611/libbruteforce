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
//! string. These benchmarks measure exactly those steps, single-threaded and in
//! isolation, so they are fast and low-noise and reliably show the effect of
//! changes to the candidate-generation code.
//!
//! The `hashing` group is included for context only: it is the dominant
//! per-candidate cost in a real run, so the generation numbers should always be
//! weighed against it.

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use libbruteforce::TargetHashInput;
use libbruteforce::bench_internals::{indices_create, indices_increment_by, indices_to_string};
use libbruteforce::hash_fncs::sha256_hashing;
use libbruteforce::symbols::Builder;
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

criterion_group!(benches, bench_candidate_generation, bench_hashing);
criterion_main!(benches);
