use std::collections::BTreeSet;

use crate::symbols::{
    ALL_OTHER_SPECIAL_CHARS,
    COMMON_SPECIAL_CHARS,
    DIGITS,
    LC_LETTERS,
    LC_UMLAUTS,
    UC_LETTERS,
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
    pub fn new() -> Self {
        Self::default()
    }

    /// Shorthand for all possible symbols.
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
    pub fn with_digits(mut self) -> Self {
        self.chars.extend(&DIGITS);
        self
    }

    /// Letters A-z. Shorthand for `with_lc_letters()` and `with_uc_letters()`.
    pub fn with_letters(self) -> Self {
        self.with_lc_letters().with_uc_letters()
    }

    /// Letters ÄÖÜäöü. Shorthand for `with_lc_umlauts()` and `with_uc_umlauts()`.
    pub fn with_umlauts(self) -> Self {
        self.with_lc_umlauts().with_uc_umlauts()
    }

    /// Letters A-Z
    pub fn with_uc_letters(mut self) -> Self {
        self.chars.extend(&UC_LETTERS);
        self
    }

    /// Letters a-z
    pub fn with_lc_letters(mut self) -> Self {
        self.chars.extend(&LC_LETTERS);

        self
    }

    /// Letters ÄÖÜ
    pub fn with_uc_umlauts(mut self) -> Self {
        self.chars.extend(&UC_UMLAUTS);
        self
    }

    /// Letters äöü
    pub fn with_lc_umlauts(mut self) -> Self {
        self.chars.extend(&LC_UMLAUTS);
        self
    }

    /// Common special chars on QWERTZ layout, see `COMMON_SPECIAL_CHARS`.
    pub fn with_common_special_chars(mut self) -> Self {
        self.chars.extend(&COMMON_SPECIAL_CHARS);
        self
    }

    /// Other special chars on QWERTZ layout, see `ALL_OTHER_SPECIAL_CHARS`.
    pub fn with_all_other_special_chars(mut self) -> Self {
        self.chars.extend(&ALL_OTHER_SPECIAL_CHARS);
        self
    }

    /// Shorthand for `with_all_other_special_chars` and `with_common_special_chars`.
    pub fn with_all_special_chars(self) -> Self {
        self.with_common_special_chars()
            .with_all_other_special_chars()
    }

    pub fn with_char(mut self, c: char) -> Self {
        self.chars.insert(c);
        self
    }

    /// Builds the alphabet.
    pub fn build(self) -> Box<[char]> {
        if self.chars.is_empty() {
            panic!("Alphabet is empty!")
        }
        self.chars
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::builder::Builder;
    use crate::symbols::{
        ALL_OTHER_SPECIAL_CHARS,
        COMMON_SPECIAL_CHARS,
        DIGITS,
        LC_LETTERS,
        LC_UMLAUTS,
        UC_LETTERS,
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
        Builder::new().build();
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
