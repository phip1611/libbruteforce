use crate::transformation_fns::TransformationFn;

pub static NO_HASHING: TransformationFn = identity;

fn identity(input: &String) -> String {
    String::from(input)
}
