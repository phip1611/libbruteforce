//! This module describes all structures and functions related with "indices".
//! Indices is the representation of a possible combination in our cracking
//! process. The indices array has the length of the maximum crack size.
//! The indices describe the index inside the alphabet. Whats special here
//! is that indices start at -1. -1 means that the current guesses are not
//! that long. E.g. means [-1, -1, -1, 2] that we so far just tried combinations
//! with one digit. Once a index goes from -1 to 0 it never goes back.

/// Initializes the array with -1 in each field and returns it.
/// Array is created on the heap.
///
/// The array represents the indices of the symbols in the current
/// iteration trying to crack the password.
///
/// Example: If our alphabet is Σ={a,b,c}, our maximum password
/// length is 5 and our current attempt is `,,a,c,b` then the
/// indices will be `-1,-1,0,2,1`. Values will never ever
/// go back to "-1" once been at 0 because we can't have empty
/// slots inside a word (they shall be marked with a space in
/// the alphabet).
pub fn indices_create(max_length: u32, min_length: u32) -> Box<[isize]> {
    if min_length > max_length {
        panic!("max_length must be >= min_length")
    }
    // -1 means no symbol yet
    let mut slice = vec![-1; max_length as usize].into_boxed_slice();
    for i in 0..min_length {
        let index = (max_length - 1 - i) as usize;
        slice[index] = 0; // from -1 to 0
    }
    slice
}

/// Transforms the indices array into a string using the alphabet.
/// Empty slots will be skipped. They contain the empty word.
#[inline] // small but notable performance gain
pub fn indices_to_string(buf: &mut String, alphabet: &[char], indices: &[isize]) {
    // clear keeps the capacity
    buf.clear();
    for index in indices {
        // skip empty fields. -1 means nothing, not " "
        if *index != -1 {
            let symbol = alphabet[*index as usize];
            buf.push(symbol);
        }
    }
}

/// Increments the indices array by a given number.
/// Returns Ok() on next number or Err() on final
/// overflow (=done).
#[inline]
pub fn indices_increment_by(
    alphabet: &[char],
    indices: &mut [isize],
    add_value: usize,
) -> Result<(), &'static str> {
    if add_value == 0 {
        // Nothing to do
        return Ok(());
    }

    // The carry from the last iteration; in the first iteration the carry
    // is the add_value; in each further iteration its the actual carry
    let mut carry = add_value;
    for i in 0..indices.len() {
        // we go from left to right
        let position = indices.len() - 1 - i;
        if carry == 0 {
            // done, no more carry to bring to the next position
            break;
        }

        // the current index at this position in the indices array
        let current_value = indices[position];
        let mut new_value = current_value + carry as isize;

        // out of bounds? modulo!
        if new_value >= alphabet.len() as isize {
            // carry for next position/next iteration
            carry = new_value as usize / alphabet.len();
            new_value %= (alphabet.len()) as isize;
        } else {
            carry = 0;
        }

        indices[position] = new_value;
    }

    if carry == 0 {
        Ok(())
    } else {
        // at the end its not the original state or the maximum value but some
        // invalid value
        Err("Overflow detected! Data/state is now invalid and no longer reliable!")
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::combinations_count;

    use super::*;

    #[test]
    fn test_create_indices_arr() {
        let arr = indices_create(3, 0);
        assert_eq!(arr[0], -1);
        assert_eq!(arr[1], -1);
        assert_eq!(arr[2], -1);

        let arr = indices_create(3, 2);
        // expected: [-1; 0; 0]
        assert_eq!(arr[0], -1);
        assert_eq!(arr[1], 0);
        assert_eq!(arr[2], 0);

        let arr = indices_create(3, 3);
        // expected: [-1; 0; 0]
        assert_eq!(arr[0], 0);
        assert_eq!(arr[1], 0);
        assert_eq!(arr[2], 0);
    }

    #[test]
    #[should_panic]
    fn test_create_indices_arr_panic() {
        indices_create(0, 1);
    }

    #[test]
    fn test_get_word_as_string_1() {
        let alphabet: Box<[char]> = Box::from(['a', 'b', 'c']);
        let mut arr = indices_create(5, 0);
        arr[2] = 1;
        arr[3] = 2;
        arr[4] = 0;
        let mut str_buf = String::new();
        indices_to_string(&mut str_buf, &alphabet, &arr);
        assert_eq!(str_buf, "bca", "Strings should equal")
    }

    #[test]
    fn test_get_word_as_string_2() {
        let alphabet: Box<[char]> = Box::from(['a', 'b', 'c']);
        let mut arr = indices_create(5, 0);
        arr[0] = 1;
        arr[1] = 1;
        arr[2] = 1;
        arr[3] = 2;
        arr[4] = 0;
        let mut str_buf = String::new();
        indices_to_string(&mut str_buf, &alphabet, &arr);
        assert_eq!(str_buf, "bbbca", "Strings should equal")
    }

    #[test]
    fn test_increment_indices_array_add1_overflow() {
        let alphabet: Box<[char]> = Box::from(['0', '1']);
        let mut arr = indices_create(5, 0);
        arr[3] = 1;
        arr[4] = 1;
        indices_increment_by(&alphabet, &mut arr, 1).unwrap();
        assert_eq!(arr[0], -1, "after '11' comes '000'");
        assert_eq!(arr[1], -1, "after '11' comes '000'");
        assert_eq!(arr[2], 0, "after '11' comes '000'");
        assert_eq!(arr[3], 0, "after '11' comes '000'");
        assert_eq!(arr[4], 0, "after '11' comes '000'");
    }

    #[test]
    fn test_increment_indices_array_add1() {
        let alphabet: Box<[char]> = Box::from(['a', 'b', 'c', 'd', 'e', 'f']);
        let mut arr = indices_create(5, 0);
        arr[2] = 3;
        arr[3] = 5;
        arr[4] = 5;
        indices_increment_by(&alphabet, &mut arr, 1).unwrap();
        assert_eq!(arr[2], 4, "after 'ffd' comes 'ffe'");
        assert_eq!(arr[3], 0, "after 'ffd' comes 'ffe'");
        assert_eq!(arr[4], 0, "after 'ffd' comes 'ffe'");
    }

    #[test]
    fn test_increment_indices_array_add1_initial() {
        let alphabet: Box<[char]> = Box::from(['a', 'b']);
        let mut arr = indices_create(5, 0);
        indices_increment_by(&alphabet, &mut arr, 1).unwrap();
        assert_eq!(arr[4], 0, "after () comes 'a'");
    }

    #[test]
    fn test_increment_indices_array_total_overflow() {
        let alphabet: Box<[char]> = Box::from(['a', 'b', 'c', 'd', 'e', 'f']);
        let mut arr = indices_create(3, 0);
        arr[0] = 5;
        arr[1] = 5;
        arr[2] = 5;
        assert!(
            indices_increment_by(&alphabet, &mut arr, 1).is_err(),
            "fff with length 3 should not be incrementable!"
        )
    }

    #[test]
    fn test_increment_indices_to_upper_bound() {
        let len = 3;
        let alphabet: Box<[char]> = Box::from(['a', 'b', 'c']);
        let mut indices = indices_create(len, 0);
        // should make -1 -1 -1 to 2 2 2
        // minus one because we are already at the first element (-1, -1, -1)
        let steps = combinations_count(&alphabet, len, 0) - 1;
        indices_increment_by(&alphabet, &mut indices, steps).unwrap();
        for i in 0..len {
            assert_eq!(indices[i as usize], (alphabet.len() - 1) as isize)
        }
    }
}
