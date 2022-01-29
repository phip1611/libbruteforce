// reexport
pub use crate::transform_fns::identity::no_hashing;
pub use crate::transform_fns::md5::{md5_hashing, Md5Hash};
pub use crate::transform_fns::sha1::{sha1_hashing, Sha1Hash};
pub use crate::transform_fns::sha256::{sha256_hashing, Sha256Hash};

mod identity;
mod md5;
mod sha1;
mod sha256;

#[cfg(test)]
mod tests {
    // use super::*;
}
