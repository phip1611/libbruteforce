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
//! Exports the hashing algorithm [`no_hashing`].
use crate::{TargetHashAndHashFunction, TargetHashInput};

/// Returns a [`TargetHashAndHashFunction`] object that does no hashing but works
/// on plain text/strings. Useful for debugging and testing.
/// It gets initialized with a object of type [`TargetHashInput`].
pub fn no_hashing(input: TargetHashInput) -> TargetHashAndHashFunction<String> {
    TargetHashAndHashFunction::new(input, identity, identity, string_ref_to_string)
}

fn identity(input: &str) -> String {
    String::from(input)
}

#[allow(clippy::ptr_arg)]
fn string_ref_to_string(input: &String) -> String {
    String::from(input)
}
