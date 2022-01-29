mod basic;
mod internal;
mod target_hash;

use crate::CrackTarget;

pub use basic::BasicCrackParameter;
pub use target_hash::*;

pub(crate) use internal::InternalCrackParameter;

/// Crack parameter for [`crack`]. It combines the basic struct [`BasicCrackParameter`]
/// with the generic [`TargetHashAndHashFunction`]. This separation exists so that
/// hash selection functions can be written more convenient.
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
pub struct CrackParameter<T: CrackTarget> {
    /// Basic parameters.
    basic: BasicCrackParameter,
    /// Target hash and hashing algorithm.
    target_hash_and_hash_fnc: TargetHashAndHashFunction<T>,
}

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
    pub fn new(basic: BasicCrackParameter, crack_info: TargetHashAndHashFunction<T>) -> Self {
        Self {
            basic,
            target_hash_and_hash_fnc: crack_info,
        }
    }

    /// Convenient wrapper for [`BasicCrackParameter::alphabet`].
    pub fn alphabet(&self) -> &[char] {
        self.basic.alphabet()
    }

    /// Convenient wrapper for [`BasicCrackParameter::max_length`].
    pub fn max_length(&self) -> u32 {
        self.basic.max_length()
    }

    /// Convenient wrapper for [`BasicCrackParameter::min_length`].
    pub fn min_length(&self) -> u32 {
        self.basic.min_length()
    }

    /// Convenient wrapper for [`BasicCrackParameter::fair_mode`].
    pub fn fair_mode(&self) -> bool {
        self.basic.fair_mode()
    }

    pub fn target_hash_and_hash_fnc(&self) -> &TargetHashAndHashFunction<T> {
        &self.target_hash_and_hash_fnc
    }
}
