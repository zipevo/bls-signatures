use std::ffi::c_void;

use bls_dash_sys::{
    CBIP32ChainCodeFree, CBIP32ChainCodeIsEqual, CBIP32ChainCodeSerialize,
    CBIP32ExtendedPrivateKeyFree, CBIP32ExtendedPrivateKeyFromBytes,
    CBIP32ExtendedPrivateKeyFromSeed, CBIP32ExtendedPrivateKeyGetChainCode,
    CBIP32ExtendedPrivateKeyGetExtendedPublicKey, CBIP32ExtendedPrivateKeyGetPrivateKey,
    CBIP32ExtendedPrivateKeyGetPublicKey, CBIP32ExtendedPrivateKeyIsEqual,
    CBIP32ExtendedPrivateKeyPrivateChild, CBIP32ExtendedPrivateKeyPublicChild,
    CBIP32ExtendedPrivateKeySerialize, CBIP32ExtendedPublicKeyFree,
    CBIP32ExtendedPublicKeyFromBytes, CBIP32ExtendedPublicKeyGetChainCode,
    CBIP32ExtendedPublicKeyIsEqual, CBIP32ExtendedPublicKeyPublicChild,
    CBIP32ExtendedPublicKeySerialize,
};

use crate::{
    bip32::{chain_code::ChainCode, ExtendedPublicKey},
    utils::{c_err_to_result, SecureBox},
    BlsError, G1Element, PrivateKey,
};

pub const BIP32_EXTENDED_PRIVATE_KEY_SIZE: usize = 93;

#[derive(Debug)]
pub struct ExtendedPrivateKey {
    c_extended_private_key: *mut c_void,
}

impl PartialEq for ExtendedPrivateKey {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            CBIP32ExtendedPrivateKeyIsEqual(
                self.c_extended_private_key,
                other.c_extended_private_key,
            )
        }
    }
}

impl Eq for ExtendedPrivateKey {}

impl ExtendedPrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BlsError> {
        if bytes.len() != BIP32_EXTENDED_PRIVATE_KEY_SIZE {
            return Err(BlsError {
                msg: format!(
                    "Extended Private Key size must be {}, got {}",
                    BIP32_EXTENDED_PRIVATE_KEY_SIZE,
                    bytes.len()
                ),
            });
        }
        Ok(ExtendedPrivateKey {
            c_extended_private_key: c_err_to_result(|did_err| unsafe {
                CBIP32ExtendedPrivateKeyFromBytes(bytes.as_ptr() as *const _, did_err)
            })?,
        })
    }

    pub fn from_seed(bytes: &[u8]) -> Result<Self, BlsError> {
        Ok(ExtendedPrivateKey {
            c_extended_private_key: c_err_to_result(|did_err| unsafe {
                CBIP32ExtendedPrivateKeyFromSeed(bytes.as_ptr() as *const _, did_err)
            })?,
        })
    }

    pub(crate) fn private_child_with_legacy_flag(&self, index: u32, legacy: bool) -> Self {
        ExtendedPrivateKey {
            c_extended_private_key: unsafe {
                CBIP32ExtendedPrivateKeyPrivateChild(self.c_extended_private_key, index, legacy)
            },
        }
    }

    pub fn private_child(&self, index: u32) -> Self {
        self.private_child_with_legacy_flag(index, false)
    }

    pub fn public_child(&self, index: u32) -> ExtendedPublicKey {
        ExtendedPublicKey {
            c_extended_public_key: unsafe {
                CBIP32ExtendedPrivateKeyPublicChild(self.c_extended_private_key, index)
            },
        }
    }

    pub(crate) fn extended_public_key_with_legacy_flag(
        &self,
        legacy: bool,
    ) -> Result<ExtendedPublicKey, BlsError> {
        Ok(ExtendedPublicKey {
            c_extended_public_key: c_err_to_result(|did_err| unsafe {
                CBIP32ExtendedPrivateKeyGetExtendedPublicKey(
                    self.c_extended_private_key,
                    legacy,
                    did_err,
                )
            })?,
        })
    }

    pub fn extended_public_key(&self) -> Result<ExtendedPublicKey, BlsError> {
        self.extended_public_key_with_legacy_flag(false)
    }

    pub fn public_key(&self) -> Result<G1Element, BlsError> {
        Ok(G1Element {
            c_element: c_err_to_result(|did_err| unsafe {
                CBIP32ExtendedPrivateKeyGetPublicKey(self.c_extended_private_key, did_err)
            })?,
        })
    }

    pub fn private_key(&self) -> PrivateKey {
        PrivateKey {
            c_private_key: unsafe {
                CBIP32ExtendedPrivateKeyGetPrivateKey(self.c_extended_private_key)
            },
        }
    }

    pub fn serialize(&self) -> SecureBox {
        unsafe {
            let secalloc_ptr = CBIP32ExtendedPrivateKeySerialize(self.c_extended_private_key);
            SecureBox::from_ptr(secalloc_ptr as *mut u8, BIP32_EXTENDED_PRIVATE_KEY_SIZE)
        }
    }

    pub fn chain_code(&self) -> ChainCode {
        ChainCode {
            c_chain_code: unsafe {
                CBIP32ExtendedPrivateKeyGetChainCode(self.c_extended_private_key)
            },
        }
    }
}

impl Drop for ExtendedPrivateKey {
    fn drop(&mut self) {
        unsafe { CBIP32ExtendedPrivateKeyFree(self.c_extended_private_key) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize() {
        let seed = b"seedweedseedweedseedweedseedweed";
        let private_key =
            ExtendedPrivateKey::from_seed(seed).expect("cannot generate extended private key");

        let private_key_bytes = private_key.serialize();
        let private_key_2 = ExtendedPrivateKey::from_bytes(private_key_bytes.as_ref())
            .expect("cannot deserialize extended private key");

        assert_eq!(private_key, private_key_2);
        assert_eq!(private_key.private_key(), private_key_2.private_key());
        assert_eq!(private_key.public_key(), private_key_2.public_key());
    }

    #[test]
    fn hierarchical_deterministic_keys() {
        let seed = b"seedweedseedweedseedweedseedweed";
        let private_key =
            ExtendedPrivateKey::from_seed(seed).expect("cannot generate extended private key");
        let public_key = private_key
            .extended_public_key()
            .expect("cannot get extended public key");

        let private_child = private_key.private_child(1337);
        let private_grandchild = private_child.private_child(420);

        let public_child = public_key.public_child(1337);
        let public_grandchild = public_child.public_child(420);

        assert_eq!(
            public_grandchild,
            private_grandchild
                .extended_public_key()
                .expect("cannot get extended public key")
        );
    }
}
