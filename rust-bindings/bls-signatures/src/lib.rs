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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemes::{AugSchemeMPL, Scheme};

    #[test]
    fn basic_sign() {
        let seed = b"seedweedseedweedseedweedseedweed";
        let bad_seed = b"weedseedweedseedweedseedweedseed";

        let scheme = AugSchemeMPL::new();
        let private_key =
            PrivateKey::key_gen(&scheme, seed).expect("unable to generate private key");
        let public_key = private_key
            .get_g1_element()
            .expect("unable to get public key");

        let private_key_bad =
            PrivateKey::key_gen(&scheme, bad_seed).expect("unable to generate private key");
        let public_key_bad = private_key_bad
            .get_g1_element()
            .expect("unable to get public key");

        let message = b"Evgeny owns 1337 dash no cap";

        let signature = scheme.sign(&private_key, message);
        let verify = scheme.verify(&public_key, message, &signature);
        assert!(verify);
        let verify_bad = scheme.verify(&public_key_bad, message, &signature);
        assert!(!verify_bad);
    }

    #[test]
    fn bad_seed() {
        let seed = b"lol";
        let scheme = AugSchemeMPL::new();
        let private_key = PrivateKey::key_gen(&scheme, seed);

        assert!(matches!(
            private_key,
            Err(BlsError { msg }) if msg == "Seed size must be at least 32 bytes"
        ));
    }
}
