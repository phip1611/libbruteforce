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
use libbruteforce::hash_fncs::{md5_hashing, no_hashing, sha1_hashing, sha256_hashing};
use libbruteforce::{BasicCrackParameter, CrackParameter, TargetHashInput, crack, symbols};

/// This code is the result of hours of struggling when I tried to generify the selection
/// of the hashing algorithm. The problem is that each hashing algorithm operates
/// on a generic trait type. I found no solution for a `select_algorithm()`
/// function. To enable an user selection of the user algorithm, the best solution
/// currently is to hard code all possible variants and invoke them conditionally
/// as shown below. Because the Crack Result is not generic, this works.
fn main() {
    let algo = "md5";
    let alphabet = symbols::Builder::new().with_digits().build();
    let basic_param = BasicCrackParameter::new(alphabet, 5, 0, true);
    let user_input = "my-awesome-hash";
    let res = match algo {
        "md5" => crack(CrackParameter::new(
            basic_param,
            md5_hashing(TargetHashInput::Plaintext(user_input)),
        )),
        "sha1" => crack(CrackParameter::new(
            basic_param,
            sha1_hashing(TargetHashInput::Plaintext(user_input)),
        )),
        "sha256" => crack(CrackParameter::new(
            basic_param,
            sha256_hashing(TargetHashInput::Plaintext(user_input)),
        )),
        "identity" => crack(CrackParameter::new(
            basic_param,
            no_hashing(TargetHashInput::Plaintext(user_input)),
        )),
        _ => panic!("invalid algorithm"),
    };
    println!("{:#?}", res);
}
