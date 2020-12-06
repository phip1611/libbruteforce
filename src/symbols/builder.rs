use crate::symbols::{DIGITS, UC_LETTERS, LC_LETTERS, UC_UMLAUTS, LC_UMLAUTS, COMMON_SPECIAL_CHARS, ALL_OTHER_SPECIAL_CHARS};

/// This is a builder to help you in a convenient way
/// to build a alphabet based on common chars built-in
/// into the library.
///
/// This builder is optional and not required to build
/// a alphabet for the lib.
pub struct Builder {
    chars: Vec<char>,
    added_digits: bool,
    added_lc_letters: bool,
    added_uc_letters: bool,
    added_lc_umlauts: bool,
    added_uc_umlauts: bool,
    added_common_special_chars: bool,
    added_all_other_special_chars: bool,
}

impl Builder {

    /// Creates a new empty builder instance.
    pub fn new() -> Builder {
        Builder {
            chars: vec![],
            added_digits: false,
            added_lc_letters: false,
            added_uc_letters: false,
            added_lc_umlauts: false,
            added_uc_umlauts: false,
            added_common_special_chars: false,
            added_all_other_special_chars: false,
        }
    }

    /// Shorthand for all possible symbols.
    pub fn full(mut self) -> Builder {
        // first letters and digits because they are more common
        self.with_uc_letters()
            .with_lc_letters()
            .with_digits()
            .with_all_special_chars()
            .with_uc_umlauts()
            .with_lc_umlauts()
    }

    /// Digits 0 to 9
    pub fn with_digits(mut self) -> Builder {
        if !self.added_digits {
            self.added_digits = true;
            self.chars.extend_from_slice(&DIGITS);
        }
        self
    }

    /// Letters A-z. Shorthand for `with_lc_letters()` and `with_uc_letters()`.
    pub fn with_letters(mut self) -> Builder {
        self.with_lc_letters()
            .with_uc_letters()
    }

    /// Letters ÄÖÜäöü. Shorthand for `with_lc_umlauts()` and `with_uc_umlauts()`.
    pub fn with_umlauts(mut self) -> Builder {
        self.with_lc_umlauts()
            .with_uc_umlauts()
    }

    /// Letters A-Z
    pub fn with_uc_letters(mut self) -> Builder {
        if !self.added_uc_letters {
            self.added_uc_letters = true;
            self.chars.extend_from_slice(&UC_LETTERS);
        }
        self
    }

    /// Letters a-z
    pub fn with_lc_letters(mut self) -> Builder {
        if !self.added_lc_letters {
            self.added_lc_letters = true;
            self.chars.extend_from_slice(&LC_LETTERS);
        }
        self
    }

    /// Letters ÄÖÜ
    pub fn with_uc_umlauts(mut self) -> Builder {
        if !self.added_uc_umlauts {
            self.added_uc_umlauts = true;
            self.chars.extend_from_slice(&UC_UMLAUTS);
        }
        self
    }

    /// Letters äöü
    pub fn with_lc_umlauts(mut self) -> Builder {
        if !self.added_lc_umlauts {
            self.added_lc_umlauts = true;
            self.chars.extend_from_slice(&LC_UMLAUTS);
        }
        self
    }

    /// Common special chars on QWERTZ layout, see `COMMON_SPECIAL_CHARS`.
    pub fn with_common_special_chars(mut self) -> Builder {
        if !self.added_common_special_chars {
            self.added_common_special_chars = true;
            self.chars.extend_from_slice(&COMMON_SPECIAL_CHARS);
        }
        self
    }

    /// Other special chars on QWERTZ layout, see `ALL_OTHER_SPECIAL_CHARS`.
    pub fn with_all_other_special_chars(mut self) -> Builder {
        if !self.added_all_other_special_chars {
            self.added_all_other_special_chars = true;
            self.chars.extend_from_slice(&ALL_OTHER_SPECIAL_CHARS);
        }
        self
    }

    /// Shorthand for `with_all_other_special_chars` and `with_common_special_chars`.
    pub fn with_all_special_chars(mut self) -> Builder {
        self.with_common_special_chars()
            .with_all_other_special_chars()
    }

    /// Builds the alphabet.
    pub fn build(self) -> Box<[char]> {
        if self.chars.is_empty() {
            panic!("Alphabet is empty!")
        }
        self.chars.into_boxed_slice()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::builder::Builder;
    use crate::symbols::{DIGITS, LC_LETTERS, UC_LETTERS, LC_UMLAUTS, UC_UMLAUTS, ALL_OTHER_SPECIAL_CHARS, COMMON_SPECIAL_CHARS};

    #[test]
    fn test_build() {
        let mut builder = Builder::new();
        let alphabet = builder.with_digits()
            .with_uc_letters()
            .with_lc_letters()
            .with_uc_umlauts()
            .with_lc_umlauts()
            .with_common_special_chars()
            .with_all_other_special_chars()
            .build();
        let alphabet_2 = Builder::new().full().build();

        assert_eq!(alphabet.len(), DIGITS.len() + LC_LETTERS.len() + UC_LETTERS.len() +
            LC_UMLAUTS.len() + UC_UMLAUTS.len() + ALL_OTHER_SPECIAL_CHARS.len() +
            COMMON_SPECIAL_CHARS.len()
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
        let mut builder = Builder::new();
        let alphabet = builder
            .with_common_special_chars()
            .with_all_other_special_chars()
            .build();

        let mut builder = Builder::new();
        let alphabet2 = builder
            .with_all_special_chars()
            .build();

        assert_eq!(alphabet.len(), alphabet2.len(), "all special chars must be contained!");
    }
}