use crate::transform_fns::TransformFn;

/// No hashing. Useful for debugging and testing.
pub const NO_HASHING: TransformFn<String> = identity;

fn identity(input: &str) -> String {
    String::from(input)
}
