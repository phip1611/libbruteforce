use libbruteforce::{BasicCrackParameter, crack, CrackParameter, symbols, TargetHashInput};
use libbruteforce::hash_fncs::{md5_hashing, no_hashing, sha1_hashing, sha256_hashing};

/// This code is the result of hours of struggling when I tried to generify the selection
/// of the hashing algorithm. The problem is that each hashing algorithm operates
/// on a generic trait type. I found no solution for a `select_algorithm()`
/// function. To enable an user selection of the user algorithm, the best solution
/// currently is to hard code all possible variants and invoke them conditionally
/// as shown below. Because the Crack Result is not generic, this works.
fn main() {
    let algo = "md5";
    let alphabet = symbols::Builder::new().with_digits().build();
    let basic_param = BasicCrackParameter::new(
        alphabet,
        5,
        0,
        true
    );
    let user_input = "my-awesome-hash";
    let res = match algo {
        "md5" => {
            crack(
                CrackParameter::new(basic_param, md5_hashing(TargetHashInput::Plaintext(user_input)))
            )
        }
        "sha1" => {
            crack(
                CrackParameter::new(basic_param, sha1_hashing(TargetHashInput::Plaintext(user_input)))
            )
        }
        "sha256" => {
            crack(
                CrackParameter::new(basic_param, sha256_hashing(TargetHashInput::Plaintext(user_input)))
            )
        }
        "identity" => {
            crack(
                CrackParameter::new(basic_param, no_hashing(TargetHashInput::Plaintext(user_input)))
            )
        }
        _ => panic!("invalid algorithm")
    };
    println!("{:#?}", res);
}
