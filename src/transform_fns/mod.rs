// reexport
pub use crate::transform_fns::identity::*;
pub use crate::transform_fns::md5::*;
pub use crate::transform_fns::sha1::*;
pub use crate::transform_fns::sha256::*;

mod identity;
mod md5;
mod sha1;
mod sha256;

/// This type describes functions used to transform the current guess during the bruteforce run
/// before it is compared to the target value. This can be a hashing algorithm, such as
/// identity, md5, or sha256. `T` is of type [`crate::CrackTarget`].
pub type TransformFn<T> = fn(&str) -> T;

#[cfg(test)]
mod tests {
    use crate::transform_fns::identity::NO_HASHING;
    use crate::transform_fns::md5::MD5_HASHING;
    use crate::transform_fns::sha1::SHA1_HASHING;
    use crate::transform_fns::sha256::SHA256_HASHING;
    use crate::transform_fns::{str_to_md5_hash, str_to_sha1_hash, str_to_sha256_hash};

    #[test]
    fn test_identity() {
        let input = String::from("Hello World");
        let expected = input.clone();
        let transformed = NO_HASHING(&input);
        assert!(expected.eq(&transformed), "Both strings should equal!")
    }

    #[test]
    fn test_md5() {
        let input = String::from("Hello World");
        let expected = str_to_md5_hash("b10a8db164e0754105b7a99be72e3fe5");
        let transformed = MD5_HASHING(&input);
        assert!(expected.eq(&transformed), "MD5 hashes should equal!")
    }

    #[test]
    fn test_sha1() {
        let input = String::from("Hello World");
        let expected = str_to_sha1_hash("0a4d55a8d778e5022fab701977c5d840bbc486d0");
        let transformed = SHA1_HASHING(&input);
        assert!(expected.eq(&transformed), "Sha1 hashes should equal!")
    }

    #[test]
    fn test_sha256() {
        let input = String::from("Hello World");
        let expected =
            str_to_sha256_hash("a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e");
        let transformed = SHA256_HASHING(&input);
        assert!(expected.eq(&transformed), "Sha256 hashes should equal!")
    }
}
