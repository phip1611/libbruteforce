//! This module contains prebuilt alphabets/symbols that you can use.

mod builder;

// export
pub use builder::Builder;

/// Latin Digits
pub static DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Latin letters (upper case)
pub static UC_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Latin letters (lower case)
pub static LC_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Common german umlauts (upper case)
pub static UC_UMLAUTS: [char; 3] = ['Ä', 'Ö', 'Ü'];

/// Common german umlauts (lower case)
pub static LC_UMLAUTS: [char; 3] = ['ä', 'ö', 'ü'];

/// Common special chars on a qwertz keyboard
pub static COMMON_SPECIAL_CHARS: [char; 16] = [
    ' ', '!', '$', '%', '&', '/', '(', ')', '=', '?', '+', '#', '-', '.', ',', 'ß',
];

/// all special chars on a common qwertz keyboard
pub static ALL_OTHER_SPECIAL_CHARS: [char; 26] = [
    '_', ':', ';', '<', '>', '|', '\'', '*', '~', '`', '´', '\\', '^', '°', '(', ')', '{', '}',
    '[', ']', '€', '@', '"', '§', '/', 'µ',
];


/// Calculates the amount of possible permutations if n symbols are given and m slots are available.
/// This solutions counts in that the value can be zero-length, one-length and so on.
pub fn combinations_count(alphabet: &[char], max_length: u32, min_length: u32) -> usize {
    if min_length > max_length {
        panic!("max_length must be >= min_length")
    }
    let mut sum = 0;
    for i in min_length..(max_length + 1) {
        sum += alphabet.len().pow(i);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_count() {
        let alphabet1: Box<[char]> = Box::from(['a']);
        let alphabet2: Box<[char]> = Box::from(['a', 'b', 'c']);
        let alphabet3: Box<[char]> = Box::from(['a', 'b']);
        let alphabet4: Box<[char]> = Box::from([]);
        let alphabet5: Box<[char]> = Box::from(DIGITS);
        assert_eq!(
            combinations_count(&alphabet1, 3, 0),
            4,
            "1 symbol and a maximum length of 3"
        );
        assert_eq!(
            combinations_count(&alphabet2, 1, 0),
            4,
            "3 symbols and a maximum length of 1"
        );
        assert_eq!(
            combinations_count(&alphabet3, 3, 0),
            15,
            "3 symbols and a maximum length of 3"
        );
        assert_eq!(combinations_count(&alphabet4, 0, 0), 1, "0 symbols");
        assert_eq!(combinations_count(&alphabet4, 0, 0), 1, "0 symbols");
        assert_eq!(combinations_count(&alphabet5, 4, 4), 10_000);
    }

    #[test]
    #[should_panic]
    fn test_combinations_count_panic() {
        let alphabet: Box<[char]> = Box::from(['a']);
        assert_eq!(
            combinations_count(&alphabet, 0, 1),
            0,
            "min length must be <= max length"
        );
    }
}
