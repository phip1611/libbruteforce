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
1.56.1

#### Works On
Linux, MacOS, Windows (targets with Rusts standard library)

#### Performance Hint
Always execute binaries that use this library in release mode, e.g. `cargo run --bin bench --release`.
Otherwise, the performance is really poor. For maximum Rust performance, see:
<https://deterministic.space/high-performance-rust.html>

#### Example usage
```rust
use libbruteforce::hash_fncs::sha256_hashing;
use libbruteforce::BasicCrackParameter;
use libbruteforce::{symbols, CrackParameter, TargetHashInput};
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

    // the actual cracking
    let res = libbruteforce::crack(CrackParameter::new(
        BasicCrackParameter::new(alphabet, 3, 0, true),
        sha256_hashing(TargetHashInput::HashAsStr(sha256_hash)),
    ));

    if let Some(solution) = res.solution() {
        println!("Password is: {}", solution);
        println!("Took {:.3}s", res.duration_in_seconds());
    }
}
```


README on github: <https://github.com/phip1611/bruteforcer>
Documentation on docs.rs: <https://docs.rs/libbruteforce/>
