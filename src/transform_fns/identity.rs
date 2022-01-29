use crate::TargetHashAndHashFunction;

/// Returns a [`TargetHashAndHashFunction`] object that does no hashing but works
/// on plain text/strings. Useful for debugging and testing.
/// It gets initialized with the value we want to crack. The value we want to crack
/// is a hash in string representation.
pub fn no_hashing(target_hash_as_str: &str) -> TargetHashAndHashFunction<String> {
    TargetHashAndHashFunction::new(target_hash_as_str, identity, identity)
}

fn identity(input: &str) -> String {
    String::from(input)
}
