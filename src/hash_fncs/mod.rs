// reexport
pub use crate::hash_fncs::identity::no_hashing;
pub use crate::hash_fncs::md5::{md5_hashing, Md5Hash};
pub use crate::hash_fncs::sha1::{sha1_hashing, Sha1Hash};
pub use crate::hash_fncs::sha256::{sha256_hashing, Sha256Hash};

mod identity;
mod md5;
mod sha1;
mod sha256;

#[cfg(test)]
mod tests {
    // use super::*;
}
