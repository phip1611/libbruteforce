use crate::transform_fns::TransformFn;

/// No hashing. Useful for debugging and testing.
pub static NO_HASHING: TransformFn = identity;

fn identity(input: &String) -> String {
    String::from(input)
}
