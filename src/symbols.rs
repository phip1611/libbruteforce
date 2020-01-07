/// Latin Digits
pub static DIGITS: [char; 10] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

/// Latin letters (upper and lower case)
pub static LETTERS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

/// Common umlauts (German)
pub static UMLAUTS: [char; 6] = [
    'Ä', 'Ö', 'Ü', 'ä', 'ö', 'ü'
];

/// Common special chars (qwertz keyboard)
pub static SPECIAL_CHARS: [char; 37] = [
    ' ', '-', '_', '.', ':', ',', ';', '<', '>', '|', '#', '\'', '+', '*', '~', '`', '´', '?',
    'ß', '\\', '^', '°', '(', ')', '{', '}', '[', ']', '€', '@', '!', '"', '§', '$', '%', '&',
    '/'
];

/// Builds the alphabet from the given flags.
pub fn build_alphabet(letters: bool, digits: bool, umlauts: bool, special_chars: bool) -> Box<[char]> {
    let mut symbols = vec![];
    if letters {
        symbols.extend_from_slice(&LETTERS);
    }
    if digits {
        symbols.extend_from_slice(&DIGITS);
    }
    if umlauts {
        symbols.extend_from_slice(&UMLAUTS);
    }
    if special_chars {
        symbols.extend_from_slice(&SPECIAL_CHARS);
    }
    symbols.into_boxed_slice()
}

/// Calculates the amount of possible permutations if
/// n symbols are given and m slots are available.
/// Be aware that this solutions counts in that a
/// password can be zero-length, one-length and so on.
pub fn combinations_count(alphabet: &Box<[char]>, length: u32) -> usize {
    let mut sum = 0;
    for i in 0..(length + 1) {
        sum += alphabet.len().pow(i);
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_alphabet() {
        let alphabet = build_alphabet(false, true, false, true);
        assert_eq!(alphabet.len(), 10 + 37);
        let alphabet = build_alphabet(true, true, true, true);
        assert_eq!(alphabet.len(), 10 + 52 + 6 + 37);
    }

    #[test]
    fn test_get_permutation_count() {
        let alphabet1: Box<[char]> = Box::from(['a']);
        let alphabet2: Box<[char]> = Box::from(['a', 'b', 'c']);
        let alphabet3: Box<[char]> = Box::from(['a', 'b']);
        assert_eq!(combinations_count(&alphabet1, 3), 4, "1 symbol and a maximum length of 3");
        assert_eq!(combinations_count(&alphabet2, 1), 4, "3 symbols and a maximum length of 1");
        assert_eq!(combinations_count(&alphabet3, 3), 15, "3 symbols and a maximum length of 3");
    }
}
