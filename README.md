# libbruteforce - A Rust library to brute force hashes with multiple threads

This library spawns a thread for each cpu on your system to
brute force a password/hash. It offers built-in support for MD5, SHA1,
and SHA256, but you can also provide your own hashing function as a
parameter.

You can specify your own alphabet to limit search space or use the
internally hard-coded symbols (the most common chars).

#### JUST FOR FUN!
I did this project just for fun to learn new things. Please don't
use it to do any harm to someone's privacy!

#### Maximum Performance
If you use this in a project: To let the Rust compiler produce a binary with maximum performance,
follow the steps here:
- <https://www.reddit.com/r/rust/comments/lyck1u/compiling_for_maximum_performance/>
- <https://deterministic.space/high-performance-rust.html>

#### MSRV
1.52.1

#### Works On
Linux, MacOS, Windows (targets with `std` library)

#### Hint
Always execute this library in release mode, e.g. `cargo run --bin bench --release`. Otherwise,
the performance is really poor.

#### Example usage
```rust
use libbruteforce::{symbols, transform_fns};
use libbruteforce::CrackParameter;
// If you want: add a "log"-crate compatibly logger to Cargo.toml
// like "simple_logger".
// use simple_logger::SimpleLogger;

fn main() {
    // to get information about trace! logs (like progress) on the console
    // SimpleLogger::new().init().unwrap();

    let alphabet = symbols::Builder::new().with_lc_letters().with_common_special_chars().build();
    let sha256_hash = String::from("3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d");
    // sha256("a+c")
    let res = libbruteforce::crack(
        CrackParameter::new(
            sha256_hash.clone(), alphabet, 3, 0, transform_fns::SHA256_HASHING, true,
        )
    );
    if res.is_success() {
        println!("Password is: {}", res.solution.unwrap())
    }
}
```


README on github: <https://github.com/phip1611/bruteforcer>
Documentation on docs.rs: <https://docs.rs/libbruteforce/>
