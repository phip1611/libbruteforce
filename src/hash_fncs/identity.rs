use crate::{TargetHashAndHashFunction, TargetHashInput};

/// Returns a [`TargetHashAndHashFunction`] object that does no hashing but works
/// on plain text/strings. Useful for debugging and testing.
/// It gets initialized with a object of type [`TargetHashInput`].
pub fn no_hashing(input: TargetHashInput) -> TargetHashAndHashFunction<String> {
    TargetHashAndHashFunction::new(input, identity, identity, string_ref_to_string)
}

fn identity(input: &str) -> String {
    String::from(input)
}

fn string_ref_to_string(input: &String) -> String {
    String::from(input)
}
