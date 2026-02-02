mod encryption;
mod decryption;
mod key;
mod util;
mod constants;
mod error;

pub use encryption::encrypt;
pub use decryption::decrypt;
pub use key::{random_key_128, random_key_192, random_key_256};
pub use error::{Error, Result};