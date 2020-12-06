use libbruteforce::{crack, transform_fns, symbols, CrackParameter};
use simple_logger::SimpleLogger;

fn main() {
    // to get information about trace! logs (like progress) on the console
    SimpleLogger::new().init().unwrap();

    let alphabet = symbols::Builder::new().full().build();
    // or let alphabet = symbols::build_alphabet(true, true, false, false, false, false, false)
    let input = String::from("a+c");
    // sha256 of `input`
    let sha256_hash = String::from("3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d");
    let cp = CrackParameter::new(sha256_hash.clone(), alphabet, 3, 0, transform_fns::SHA256_HASHING, true);
    let res = crack(cp);
    if res.is_success() {
        let sol = res.solution.unwrap();
        println!("Solution is: {}", sol);
        println!("found after {}s", res.seconds_as_fraction)
    } else {
        panic!("A solution must be found in the demo.")
    }
}
