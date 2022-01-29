//! The readme.md example and the example in lib.rs module description.

use libbruteforce::symbols;
use libbruteforce::transform_fns::{sha256_hashing};
use libbruteforce::CrackParameter;
use simple_logger::SimpleLogger;

/// Minimal example.
fn main() {
    // to get information about trace! logs (like progress) on the console
    SimpleLogger::new().with_utc_timestamps().init().unwrap();

    let alphabet = symbols::Builder::new()
        .with_lc_letters()
        .with_common_special_chars()
        .build();

    // sha256("a+c")
    let sha256_hash = "3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d";
    let sha256_hashing = sha256_hashing("3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d");

    // the actual cracking
    let res = libbruteforce::crack(CrackParameter::new(
        sha256_hashing,
        alphabet,
        3,
        0,
        true,
    ));

    if let Some(solution) = res.solution() {
        println!("Password is: {}", solution);
        println!("Took {:.3}s", res.seconds_as_fraction());
    }
}
