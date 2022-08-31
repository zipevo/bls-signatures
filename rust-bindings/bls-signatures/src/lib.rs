mod elements;
mod private_key;
mod schemes;
mod utils;

use std::{error::Error, fmt::Display};

pub use elements::{G1Element, G2Element};
pub use private_key::PrivateKey;

#[derive(Debug)]
pub struct BlsError {
    // Need to use owned version as each time BLS has an error its binding glue overwrites error
    // message variable.
    msg: String,
}

impl Display for BlsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for BlsError {}
