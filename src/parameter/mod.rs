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
//! This module contains types for the relatively complex input parameters of
//! [`crate::crack<T: CrackTarget>()`]. [`CrackParameter`] consists of the two
//! building blocks [`BasicCrackParameter`] and [`TargetHashAndHashFunction<T>`].
//!
//! This separation exists because experience showed that usage of the library
//! is relatively hard under certain circumstances because the highly generic
//! part of [`TargetHashAndHashFunction<T>`] can cause lots of headache when you
//! want to provide a custom selection of the hashing algorithm during runtime.
//! An example how this can be done is given in the examples inside the repository.

mod basic;
mod internal;
mod target_hash;

use crate::CrackTarget;

pub use basic::BasicCrackParameter;
pub use target_hash::*;

#[cfg(test)]
pub(crate) use internal::get_thread_count;
pub(crate) use internal::InternalCrackParameter;

/// Crack parameter for `crate::crack<T: CrackTarget>()`.
///
/// It combines the basic struct [`BasicCrackParameter`] with the generic [`TargetHashAndHashFunction`].
/// This separation exists so that hash selection functions can be written more convenient.
///
/// ```rust
/// use libbruteforce::{BasicCrackParameter, CrackParameter, TargetHashInput};
/// use libbruteforce::hash_fncs::sha256_hashing;
/// use libbruteforce::symbols;
///
/// // sha256("a+c")
/// let sha256_hash = "3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d";
/// let max_len = 3;
/// let min_len = 0;
/// let alphabet = symbols::Builder::new().with_digits().build();
///
/// let res = CrackParameter::new(
///         BasicCrackParameter::new(alphabet, max_len, min_len, false),
///         sha256_hashing(TargetHashInput::HashAsStr(sha256_hash)),
/// );
/// ```
#[derive(Debug)]
#[allow(rustdoc::private_doc_tests)]
pub struct CrackParameter<T: CrackTarget> {
    /// Basic parameters.
    basic: BasicCrackParameter,
    /// Target hash and hashing algorithm.
    target_hash_and_hash_fnc: TargetHashAndHashFunction<T>,
}

#[allow(rustdoc::private_doc_tests)]
impl<T: CrackTarget> CrackParameter<T> {
    /// Constructor.
    ///
    /// ```rust
    /// use libbruteforce::{BasicCrackParameter, CrackParameter, TargetHashInput};
    /// use libbruteforce::hash_fncs::sha256_hashing;
    /// use libbruteforce::symbols;
    ///
    /// // sha256("a+c")
    /// let sha256_hash = "3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d";
    /// let max_len = 3;
    /// let min_len = 0;
    /// let alphabet = symbols::Builder::new().with_digits().build();
    ///
    /// let res = CrackParameter::new(
    ///         BasicCrackParameter::new(alphabet, max_len, min_len, false),
    ///         sha256_hashing(TargetHashInput::HashAsStr(sha256_hash)),
    /// );
    /// ```
    pub const fn new(basic: BasicCrackParameter, crack_info: TargetHashAndHashFunction<T>) -> Self {
        Self {
            basic,
            target_hash_and_hash_fnc: crack_info,
        }
    }

    /// Convenient wrapper for [`BasicCrackParameter::alphabet`].
    pub const fn alphabet(&self) -> &[char] {
        self.basic.alphabet()
    }

    /// Convenient wrapper for [`BasicCrackParameter::max_length`].
    pub const fn max_length(&self) -> u32 {
        self.basic.max_length()
    }

    /// Convenient wrapper for [`BasicCrackParameter::min_length`].
    pub const fn min_length(&self) -> u32 {
        self.basic.min_length()
    }

    /// Convenient wrapper for [`BasicCrackParameter::fair_mode`].
    pub const fn fair_mode(&self) -> bool {
        self.basic.fair_mode()
    }

    pub const fn target_hash_and_hash_fnc(&self) -> &TargetHashAndHashFunction<T> {
        &self.target_hash_and_hash_fnc
    }
}
