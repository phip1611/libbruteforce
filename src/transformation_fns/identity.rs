use crate::transformation_fns::TransformationFn;

/// No hashing. Useful for debugging and testing.
pub static NO_HASHING: TransformationFn = identity;

fn identity(input: &String) -> String {
    String::from(input)
}
