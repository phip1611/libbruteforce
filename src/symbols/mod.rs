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
pub static ALL_OTHER_SPECIAL_CHARS: [char; 23] = [
    '_', ':', ';', '<', '>', '|', '\'', '*', '~', '`', '´', '\\', '^', '°', '{', '}', '[', ']',
    '€', '@', '"', '§', 'µ',
];

/// Calculates the amount of possible permutations if n symbols are given and m slots are available.
/// This solutions counts in that the value can be zero-length, one-length and so on.
#[must_use]
pub fn combination_count(alphabet: &[char], max_length: u32, min_length: u32) -> usize {
    if min_length > max_length {
        panic!("max_length must be >= min_length")
    }
    let mut sum = 0;
    for i in min_length..=max_length {
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
            combination_count(&alphabet1, 3, 0),
            4,
            "1 symbol and a maximum length of 3"
        );
        assert_eq!(
            combination_count(&alphabet1, 3, 3),
            1,
            "1 symbol, min=2, max=3 => 2 combination"
        );
        assert_eq!(
            combination_count(&alphabet1, 3, 3),
            1,
            "1 symbol, min=3, max=3 => 1 combination"
        );
        assert_eq!(
            combination_count(&alphabet2, 1, 0),
            4,
            "3 symbols and a maximum length of 1"
        );
        assert_eq!(
            combination_count(&alphabet2, 2, 1),
            12,
            "3 symbols, min=1, max=2 => 9 combination"
        );
        assert_eq!(
            combination_count(&alphabet3, 3, 0),
            15,
            "3 symbols and a maximum length of 3"
        );
        assert_eq!(combination_count(&alphabet4, 0, 0), 1, "0 symbols");
        assert_eq!(combination_count(&alphabet4, 0, 0), 1, "0 symbols");
        assert_eq!(combination_count(&alphabet5, 4, 4), 10_000);
    }

    #[test]
    #[should_panic]
    fn test_combinations_count_panic() {
        let alphabet: Box<[char]> = Box::from(['a']);
        assert_eq!(
            combination_count(&alphabet, 0, 1),
            0,
            "min length must be <= max length"
        );
    }
}
