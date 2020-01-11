# libbruteforce - A Rust library to brute force hashes multithreaded

This library spawns a thread for each cpu on your system to
brute force a password/hash. It offers built-in support for MD5, SHA1,
and SHA256, but you can also provide your own hashing function as a
parameter.

You can specify your own alphabet or use the internally programmed
symbols.

For example usage see https://github.com/phip1611/bruteforcer
