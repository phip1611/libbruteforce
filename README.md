# libbruteforce - A Rust library to brute force hashes multithreaded

This library spawns a thread for each cpu on your system to
brute force a password/hash. It offers built-in support for MD5, SHA1,
and SHA256, but you can also provide your own hashing function as a
parameter.

You can specify your own alphabet or use the internally programmed
symbols.

#### JUST FOR FUN!
I did this project just for fun to learn new things. Please don't
use it to do any harm to someones privacy!

#### Example usage
```rust
use libbruteforce::{symbols, crack};
use libbruteforce::transformation_fns;

fn main() {
    let alphabet = symbols::full_alphabet();
    // or let alphabet = symbols::build_alphabet(true, true, false, false, false, false, false)
    let input = String::from("a+c");
    let target = String::from("3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d");
    
    let result = crack(target, alphabet, input.len(), transformation_fns::SHA256_HASHING);
}

```

For example usage see https://github.com/phip1611/bruteforcer
