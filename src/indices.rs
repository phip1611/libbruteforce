
/// Initializes the array with -1 in each field and returns it.
/// Array is created on the heap.
///
/// The array represents the indices of the symbols in the current
/// iteration trying to crack the password.
///
/// Example: If our alphabet is Σ={a,b,c}, our maximum password
/// length is 5 and our current attempt is [,,a,c,b] then the
/// indices will be [-1,-1,0,2,1]. Values will never ever
/// go back to "-1" once been at 0 because we can't have empty
/// slots inside a word (they shall be marked with a space in
/// the alphabet).
pub fn indices_create(length: usize) -> Box<[isize]> {
    vec![-1; length].into_boxed_slice()
}

/// Transforms the indices array into a string using the alphabet.
/// Empty slots will be skipped.
pub fn indices_to_string(alphabet: &Box<[char]>, indices: &Box<[isize]>) -> String {
    let mut word = String::new();
    for i in 0..indices.len() {
        let index = indices[i];
        if index != -1 {
            // otherwise our string isn't so far that long
            let symbol = alphabet[index as usize];
            if symbol != '\0' {
                word.push(symbol)
            }
        }
    }
    word
}

/// Calculates how many fields are not "-1" aka how long the word that is represented is.
pub fn indices_word_length(indices: &Box<[isize]>) -> usize {
    let mut n = 0;
    let mut i = (indices.len() - 1) as isize;
    while i >= 0 {
        if indices[i as usize] != -1 {
            n += 1;
            i -= 1;
        } else {
            break;
        }
    }
    n
}

/// Increments the indices array by a given number.
pub fn indices_increment_by(
    alphabet: &Box<[char]>,
    indices: &mut Box<[isize]>,
    add_value: usize) -> Result<(), &'static str> {
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
            new_value = new_value % (alphabet.len()) as isize;
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
    use super::*;
    use crate::symbols::combinations_count;

    #[test]
    fn test_create_indices_arr() {
        let arr = indices_create(3);
        assert_eq!(arr[0], -1);
        assert_eq!(arr[1], -1);
        assert_eq!(arr[2], -1);
    }

    #[test]
    fn test_get_word_as_string_1() {
        let alphabet: Box<[char]> = Box::from(['a', 'b', 'c']);
        let mut arr = indices_create(5);
        arr[2] = 1;
        arr[3] = 2;
        arr[4] = 0;
        let str = indices_to_string(&alphabet, &arr);
        assert_eq!(str, "bca", "Strings should equal")
    }

    #[test]
    fn test_get_word_as_string_2() {
        let alphabet: Box<[char]> = Box::from(['a', 'b', 'c']);
        let mut arr = indices_create(5);
        arr[0] = 1;
        arr[1] = 1;
        arr[2] = 1;
        arr[3] = 2;
        arr[4] = 0;
        let str = indices_to_string(&alphabet, &arr);
        assert_eq!(str, "bbbca", "Strings should equal")
    }

    #[test]
    fn test_increment_indices_array_add1_overflow() {
        let alphabet: Box<[char]> = Box::from(['0', '1']);
        let mut arr = indices_create(5);
        arr[3] = 1;
        arr[4] = 1;
        indices_increment_by(&alphabet, &mut arr, 1);
        assert_eq!(arr[0], -1, "after '11' comes '000'");
        assert_eq!(arr[1], -1, "after '11' comes '000'");
        assert_eq!(arr[2], 0, "after '11' comes '000'");
        assert_eq!(arr[3], 0, "after '11' comes '000'");
        assert_eq!(arr[4], 0, "after '11' comes '000'");
    }

    #[test]
    fn test_increment_indices_array_add1() {
        let alphabet: Box<[char]> = Box::from(['a', 'b', 'c', 'd', 'e', 'f']);
        let mut arr = indices_create(5);
        arr[2] = 3;
        arr[3] = 5;
        arr[4] = 5;
        indices_increment_by(&alphabet, &mut arr, 1);
        assert_eq!(arr[2], 4, "after 'ffd' comes 'ffe'");
        assert_eq!(arr[3], 0, "after 'ffd' comes 'ffe'");
        assert_eq!(arr[4], 0, "after 'ffd' comes 'ffe'");
    }

    #[test]
    fn test_increment_indices_array_add1_initial() {
        let alphabet: Box<[char]> = Box::from(['a', 'b']);
        let mut arr = indices_create(5);
        indices_increment_by(&alphabet, &mut arr, 1);
        assert_eq!(arr[4], 0, "after () comes 'a'");
    }

    #[test]
    fn test_increment_indices_array_total_overflow() {
        let alphabet: Box<[char]> = Box::from(['a', 'b', 'c', 'd', 'e', 'f']);
        let mut arr = indices_create(3);
        arr[0] = 5;
        arr[1] = 5;
        arr[2] = 5;
        match indices_increment_by(&alphabet, &mut arr, 1) {
            Ok(_) => {
                assert!(false, "fff with length 3 should not be incrementable!")
            }
            _ => ()
        }
    }

    #[test]
    fn test_increment_indices_to_upper_bound() {
        const LEN: usize = 3;
        let alphabet: Box<[char]> = Box::from(['a', 'b', 'c']);
        let mut indices = indices_create(LEN);
        // should make -1 -1 -1 to 2 2 2
        // minus one because we are already at the first element (-1, -1, -1)
        let steps = combinations_count(&alphabet, LEN as u32, ) - 1;
        indices_increment_by(&alphabet, &mut indices, steps);
        for i in 0..LEN {
            assert_eq!(indices[i], (alphabet.len() - 1) as isize)
        }
    }

    #[test]
    fn test_length_of_indices_array() {
        let alphabet: Box<[char]> = Box::from(['a']);
        const LENGTH: usize = 5;
        let mut indices = indices_create(LENGTH);
        assert_eq!(0, indices_word_length(&indices));
        indices_increment_by(&alphabet, &mut indices, 1);
        assert_eq!(1, indices_word_length(&indices));
        indices_increment_by(&alphabet, &mut indices, 4);
        assert_eq!(LENGTH, indices_word_length(&indices));
    }
}