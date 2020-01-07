/// Calculates the amount of possible permutations if
/// n symbols are given and m slots are available.
/// Be aware that this solutions counts in that a
/// password can be zero-length, one-length and so on.
fn get_possible_combinations_count(symbol_count: usize, length: u32) -> usize {
  let mut sum = 0;
  for i in 0..(length + 1) {
    sum += symbol_count.pow(i);
  }
  sum
}

/// Initializes the array with -1 in each field and returns it.
/// Array is created on the heap.
///
/// The array represents the indices of the symbols in the current
/// iteration trying to crack the password.
///
/// Example: If our alphabet is Σ={a,b,c}, our maximum password
/// length is 5 and our current attempt is '[,,a,c,b]' then the
/// indices will be '[-1,-1,0,2,1]'. Values will never ever
/// go back to "-1" once been at 0 because we can't have empty
/// slots inside a word (they shall be marked with a space in
/// the alphabet).
pub fn create_indices_arr(length: usize) -> Box<[isize]> {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_permutation_count() {
    assert_eq!(get_possible_combinations_count(1, 3), 4, "1 symbol and a maximum length of 3");
    assert_eq!(get_possible_combinations_count(3, 1), 4, "3 symbols and a maximum length of 1");
    assert_eq!(get_possible_combinations_count(2, 3), 15, "3 symbols and a maximum length of 3");
  }

  #[test]
  fn test_create_indices_arr() {
    let arr = create_indices_arr(3);
    assert_eq!(arr[0], -1);
    assert_eq!(arr[1], -1);
    assert_eq!(arr[2], -1);
  }

  #[test]
  fn test_get_word_as_string_1() {
    let alphabet: Box<[char]> = Box::from(['a', 'b', 'c']);
    let mut arr = create_indices_arr(5);
    arr[2] = 1;
    arr[3] = 2;
    arr[4] = 0;
    let str = indices_to_string(&alphabet, &arr);
    assert_eq!(str, "bca", "Strings should equal")
  }

  #[test]
  fn test_get_word_as_string_2() {
    let alphabet: Box<[char]> = Box::from(['a', 'b', 'c']);
    let mut arr = create_indices_arr(5);
    arr[0] = 1;
    arr[1] = 1;
    arr[2] = 1;
    arr[3] = 2;
    arr[4] = 0;
    let str = indices_to_string(&alphabet, &arr);
    assert_eq!(str, "bbbca", "Strings should equal")
  }
}
