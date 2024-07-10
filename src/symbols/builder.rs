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
use std::collections::BTreeSet;

use crate::symbols::{
    ALL_OTHER_SPECIAL_CHARS, COMMON_SPECIAL_CHARS, DIGITS, LC_LETTERS, LC_UMLAUTS, UC_LETTERS,
    UC_UMLAUTS,
};

/// This is a builder to help you in a convenient way
/// to build a alphabet based on common chars built-in
/// into the library.
///
/// This builder is optional and not required to build
/// a alphabet for the lib.
#[derive(Default, Debug)]
pub struct Builder {
    // btree set => reproducible runs, because order of symbols is the same.
    // doesn't effect the runtime performance
    chars: BTreeSet<char>,
}

impl Builder {
    /// Creates a new empty builder instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Shorthand for all possible symbols.
    #[must_use]
    pub fn full(self) -> Self {
        // first letters and digits because they are more common
        self.with_uc_letters()
            .with_lc_letters()
            .with_digits()
            .with_all_special_chars()
            .with_uc_umlauts()
            .with_lc_umlauts()
    }

    /// Digits 0 to 9
    #[must_use]
    pub fn with_digits(mut self) -> Self {
        self.chars.extend(&DIGITS);
        self
    }

    /// Letters A-z. Shorthand for `with_lc_letters()` and `with_uc_letters()`.
    #[must_use]
    pub fn with_letters(self) -> Self {
        self.with_lc_letters().with_uc_letters()
    }

    /// Letters ÄÖÜäöü. Shorthand for `with_lc_umlauts()` and `with_uc_umlauts()`.
    #[must_use]
    pub fn with_umlauts(self) -> Self {
        self.with_lc_umlauts().with_uc_umlauts()
    }

    /// Letters A-Z
    #[must_use]
    pub fn with_uc_letters(mut self) -> Self {
        self.chars.extend(&UC_LETTERS);
        self
    }

    /// Letters a-z
    #[must_use]
    pub fn with_lc_letters(mut self) -> Self {
        self.chars.extend(&LC_LETTERS);

        self
    }

    /// Letters ÄÖÜ
    #[must_use]
    pub fn with_uc_umlauts(mut self) -> Self {
        self.chars.extend(&UC_UMLAUTS);
        self
    }

    /// Letters äöü
    #[must_use]
    pub fn with_lc_umlauts(mut self) -> Self {
        self.chars.extend(&LC_UMLAUTS);
        self
    }

    /// Common special chars on QWERTZ/Y layout, see `COMMON_SPECIAL_CHARS`.
    #[must_use]
    pub fn with_common_special_chars(mut self) -> Self {
        self.chars.extend(&COMMON_SPECIAL_CHARS);
        self
    }

    /// Other special chars on QWERTZ/Y layout, see `ALL_OTHER_SPECIAL_CHARS`.
    #[must_use]
    pub fn with_all_other_special_chars(mut self) -> Self {
        self.chars.extend(&ALL_OTHER_SPECIAL_CHARS);
        self
    }

    /// Shorthand for `with_all_other_special_chars` and `with_common_special_chars`.
    #[must_use]
    pub fn with_all_special_chars(self) -> Self {
        self.with_common_special_chars()
            .with_all_other_special_chars()
    }

    /// Adds a single char to the alphabet.
    #[must_use]
    pub fn with_char(mut self, c: char) -> Self {
        self.chars.insert(c);
        self
    }

    /// Builds the alphabet.
    #[must_use]
    pub fn build(self) -> Box<[char]> {
        if self.chars.is_empty() {
            panic!("Alphabet is empty!")
        }
        self.chars
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::builder::Builder;
    use crate::symbols::{
        ALL_OTHER_SPECIAL_CHARS, COMMON_SPECIAL_CHARS, DIGITS, LC_LETTERS, LC_UMLAUTS, UC_LETTERS,
        UC_UMLAUTS,
    };

    #[test]
    fn test_build() {
        let alphabet = Builder::new()
            .with_digits()
            .with_uc_letters()
            .with_lc_letters()
            .with_uc_umlauts()
            .with_lc_umlauts()
            .with_common_special_chars()
            .with_all_other_special_chars()
            .build();
        let alphabet_2 = Builder::new().full().build();

        assert_eq!(
            alphabet.len(),
            DIGITS.len()
                + LC_LETTERS.len()
                + UC_LETTERS.len()
                + LC_UMLAUTS.len()
                + UC_UMLAUTS.len()
                + ALL_OTHER_SPECIAL_CHARS.len()
                + COMMON_SPECIAL_CHARS.len()
        );
        assert_eq!(alphabet.len(), alphabet_2.len());
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        // test ebuild with empty alphabet
        let _ = Builder::new().build();
    }

    #[test]
    fn test_build_special_chars() {
        let alphabet = Builder::new()
            .with_common_special_chars()
            .with_all_other_special_chars()
            .build();

        let alphabet2 = Builder::new().with_all_special_chars().build();

        assert_eq!(
            alphabet.len(),
            alphabet2.len(),
            "all special chars must be contained!"
        );
    }
}
