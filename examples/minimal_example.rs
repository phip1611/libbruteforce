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
//! The readme.md example and the example in lib.rs module description.

use libbruteforce::hash_fncs::sha256_hashing;
use libbruteforce::BasicCrackParameter;
use libbruteforce::{symbols, CrackParameter, TargetHashInput};
use simple_logger::SimpleLogger;

/// Minimal example.
fn main() {
    // to get information about trace! logs (like progress) on the console
    SimpleLogger::new().with_utc_timestamps().init().unwrap();

    let alphabet = symbols::Builder::new()
        .with_lc_letters()
        .with_common_special_chars()
        .build();

    // sha256("a+c")
    let sha256_hash = "3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d";

    // the actual cracking
    let res = libbruteforce::crack(CrackParameter::new(
        BasicCrackParameter::new(alphabet, 3, 0, true),
        sha256_hashing(TargetHashInput::HashAsStr(sha256_hash)),
    ));

    if let Some(solution) = res.solution() {
        println!("Password is: {}", solution);
        println!("Took {:.3}s", res.seconds_as_fraction());
    }
}
