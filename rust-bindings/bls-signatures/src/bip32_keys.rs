use std::ffi::c_void;

use bls_dash_sys::{
    CChainCodeFree, CChainCodeIsEqual, CChainCodeSerialize, CExtendedPrivateKeyFree,
    CExtendedPrivateKeyFromBytes, CExtendedPrivateKeyFromSeed, CExtendedPrivateKeyGetChainCode,
    CExtendedPrivateKeyGetExtendedPublicKey, CExtendedPrivateKeyGetPrivateKey,
    CExtendedPrivateKeyGetPublicKey, CExtendedPrivateKeyIsEqual, CExtendedPrivateKeyPrivateChild,
    CExtendedPrivateKeyPublicChild, CExtendedPrivateKeySerialize, CExtendedPublicKeyFree,
    CExtendedPublicKeyFromBytes, CExtendedPublicKeyGetChainCode, CExtendedPublicKeyIsEqual,
    CExtendedPublicKeyPublicChild, CExtendedPublicKeySerialize,
};

use crate::{
    utils::{c_err_to_result, SecureBox},
    BlsError, G1Element, PrivateKey,
};

pub const BIP32_EXTENDED_PUBLIC_KEY_SIZE: usize = 93;
pub const BIP32_EXTENDED_PRIVATE_KEY_SIZE: usize = 77;
pub const CHAIN_CODE_SIZE: usize = 32;

#[derive(Debug)]
pub struct BIP32ExtendedPublicKey {
    c_extended_public_key: *mut c_void,
}

impl PartialEq for BIP32ExtendedPublicKey {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            CExtendedPublicKeyIsEqual(self.c_extended_public_key, other.c_extended_public_key)
        }
    }
}

impl Eq for BIP32ExtendedPublicKey {}

impl BIP32ExtendedPublicKey {
    pub(crate) fn from_bytes_with_legacy_flag(
        bytes: &[u8],
        legacy: bool,
    ) -> Result<Self, BlsError> {
        if bytes.len() != BIP32_EXTENDED_PUBLIC_KEY_SIZE {
            return Err(BlsError {
                msg: format!(
                    "Extended Public Key size must be {}, got {}",
                    BIP32_EXTENDED_PUBLIC_KEY_SIZE,
                    bytes.len()
                ),
            });
        }
        Ok(BIP32ExtendedPublicKey {
            c_extended_public_key: c_err_to_result(|did_err| unsafe {
                CExtendedPublicKeyFromBytes(bytes.as_ptr() as *const _, legacy, did_err)
            })?,
        })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BlsError> {
        Self::from_bytes_with_legacy_flag(bytes, false)
    }

    pub(crate) fn public_child_with_legacy_flag(&self, index: u32, legacy: bool) -> Self {
        BIP32ExtendedPublicKey {
            c_extended_public_key: unsafe {
                CExtendedPublicKeyPublicChild(self.c_extended_public_key, index, legacy)
            },
        }
    }

    pub fn public_child(&self, index: u32) -> Self {
        self.public_child_with_legacy_flag(index, false)
    }

    pub(crate) fn serialize_with_legacy_flag(
        &self,
        legacy: bool,
    ) -> Box<[u8; BIP32_EXTENDED_PUBLIC_KEY_SIZE]> {
        unsafe {
            let malloc_ptr = CExtendedPublicKeySerialize(self.c_extended_public_key, legacy);
            Box::from_raw(malloc_ptr as *mut _)
        }
    }

    pub fn serialize(&self) -> Box<[u8; BIP32_EXTENDED_PUBLIC_KEY_SIZE]> {
        self.serialize_with_legacy_flag(false)
    }

    pub fn get_chain_code(&self) -> ChainCode {
        ChainCode {
            c_chain_code: unsafe { CExtendedPublicKeyGetChainCode(self.c_extended_public_key) },
        }
    }
}

impl Drop for BIP32ExtendedPublicKey {
    fn drop(&mut self) {
        unsafe { CExtendedPublicKeyFree(self.c_extended_public_key) }
    }
}

#[derive(Debug)]
pub struct BIP32ExtendedPrivateKey {
    c_extended_private_key: *mut c_void,
}

impl PartialEq for BIP32ExtendedPrivateKey {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            CExtendedPrivateKeyIsEqual(self.c_extended_private_key, other.c_extended_private_key)
        }
    }
}

impl Eq for BIP32ExtendedPrivateKey {}

impl BIP32ExtendedPrivateKey {
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
        Ok(BIP32ExtendedPrivateKey {
            c_extended_private_key: c_err_to_result(|did_err| unsafe {
                CExtendedPrivateKeyFromBytes(bytes.as_ptr() as *const _, did_err)
            })?,
        })
    }

    pub fn from_seed(bytes: &[u8]) -> Result<Self, BlsError> {
        Ok(BIP32ExtendedPrivateKey {
            c_extended_private_key: c_err_to_result(|did_err| unsafe {
                CExtendedPrivateKeyFromSeed(bytes.as_ptr() as *const _, did_err)
            })?,
        })
    }

    pub(crate) fn private_child_with_legacy_flag(&self, index: u32, legacy: bool) -> Self {
        BIP32ExtendedPrivateKey {
            c_extended_private_key: unsafe {
                CExtendedPrivateKeyPrivateChild(self.c_extended_private_key, index, legacy)
            },
        }
    }

    pub fn private_child(&self, index: u32) -> Self {
        self.private_child_with_legacy_flag(index, false)
    }

    pub fn public_child(&self, index: u32) -> BIP32ExtendedPublicKey {
        BIP32ExtendedPublicKey {
            c_extended_public_key: unsafe {
                CExtendedPrivateKeyPublicChild(self.c_extended_private_key, index)
            },
        }
    }

    pub(crate) fn get_extended_public_key_with_legacy_flag(
        &self,
        legacy: bool,
    ) -> Result<BIP32ExtendedPublicKey, BlsError> {
        Ok(BIP32ExtendedPublicKey {
            c_extended_public_key: c_err_to_result(|did_err| unsafe {
                CExtendedPrivateKeyGetExtendedPublicKey(
                    self.c_extended_private_key,
                    legacy,
                    did_err,
                )
            })?,
        })
    }

    pub fn get_extended_public_key(&self) -> Result<BIP32ExtendedPublicKey, BlsError> {
        self.get_extended_public_key_with_legacy_flag(false)
    }

    pub fn get_public_key(&self) -> Result<G1Element, BlsError> {
        Ok(G1Element {
            c_element: c_err_to_result(|did_err| unsafe {
                CExtendedPrivateKeyGetPublicKey(self.c_extended_private_key, did_err)
            })?,
        })
    }

    pub fn get_private_key(&self) -> PrivateKey {
        PrivateKey {
            c_private_key: unsafe { CExtendedPrivateKeyGetPrivateKey(self.c_extended_private_key) },
        }
    }

    pub fn serialize(&self) -> SecureBox {
        unsafe {
            let secalloc_ptr = CExtendedPrivateKeySerialize(self.c_extended_private_key);
            SecureBox::from_ptr(secalloc_ptr as *mut u8, BIP32_EXTENDED_PRIVATE_KEY_SIZE)
        }
    }

    pub fn get_chain_code(&self) -> ChainCode {
        ChainCode {
            c_chain_code: unsafe { CExtendedPrivateKeyGetChainCode(self.c_extended_private_key) },
        }
    }
}

impl Drop for BIP32ExtendedPrivateKey {
    fn drop(&mut self) {
        unsafe { CExtendedPrivateKeyFree(self.c_extended_private_key) }
    }
}

#[derive(Debug)]
pub struct ChainCode {
    c_chain_code: *mut c_void,
}

impl ChainCode {
    pub fn serialize(&self) -> Box<[u8; CHAIN_CODE_SIZE]> {
        unsafe {
            let malloc_ptr = CChainCodeSerialize(self.c_chain_code);
            Box::from_raw(malloc_ptr as *mut _)
        }
    }
}

impl PartialEq for ChainCode {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CChainCodeIsEqual(self.c_chain_code, other.c_chain_code) }
    }
}

impl Eq for ChainCode {}

impl Drop for ChainCode {
    fn drop(&mut self) {
        unsafe { CChainCodeFree(self.c_chain_code) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod extended_public_key {
        use super::*;

        #[test]
        fn serialize_deserialize() {
            let seed = b"seedweedseedweedseedweedseedweed";
            let private_key = BIP32ExtendedPrivateKey::from_seed(seed)
                .expect("cannot generate extended private key");
            let public_key = private_key
                .get_extended_public_key()
                .expect("cannot get extended public key");

            let public_key_bytes = public_key.serialize();
            let public_key_2 = BIP32ExtendedPublicKey::from_bytes(public_key_bytes.as_ref())
                .expect("cannot deserialize extended public key");

            assert_eq!(public_key, public_key_2);
        }

        #[cfg(feature = "legacy")]
        #[test]
        fn serialize_deserialize_legacy() {
            let seed = b"seedweedseedweedseedweedseedweed";
            let private_key = BIP32ExtendedPrivateKey::from_seed(seed)
                .expect("cannot generate extended private key");
            let public_key = private_key
                .get_extended_public_key_legacy()
                .expect("cannot get extended public key");

            let public_key_bytes = public_key.serialize_legacy();
            let public_key_2 = BIP32ExtendedPublicKey::from_bytes_legacy(public_key_bytes.as_ref())
                .expect("cannot deserialize extended public key");

            assert_eq!(public_key, public_key_2);
        }
    }

    mod extended_private_key {
        use super::*;

        #[test]
        fn serialize_deserialize() {
            let seed = b"seedweedseedweedseedweedseedweed";
            let private_key = BIP32ExtendedPrivateKey::from_seed(seed)
                .expect("cannot generate extended private key");

            let private_key_bytes = private_key.serialize();
            let private_key_2 = BIP32ExtendedPrivateKey::from_bytes(private_key_bytes.as_ref())
                .expect("cannot deserialize extended private key");

            assert_eq!(private_key, private_key_2);
            assert_eq!(
                private_key.get_private_key(),
                private_key_2.get_private_key()
            );
            assert_eq!(private_key.get_public_key(), private_key_2.get_public_key());
        }

        #[test]
        fn hierarchical_deterministic_keys() {
            let seed = b"seedweedseedweedseedweedseedweed";
            let private_key = BIP32ExtendedPrivateKey::from_seed(seed)
                .expect("cannot generate extended private key");
            let public_key = private_key
                .get_extended_public_key()
                .expect("cannot get extended public key");

            let private_child = private_key.private_child(1337);
            let private_grandchild = private_child.private_child(420);

            let public_child = public_key.public_child(1337);
            let public_grandchild = public_child.public_child(420);

            assert_eq!(
                public_grandchild,
                private_grandchild
                    .get_extended_public_key()
                    .expect("cannot get extended public key")
            );
        }

        #[cfg(feature = "legacy")]
        #[test]
        fn hierarchical_deterministic_keys_legacy() {
            let seed = b"seedweedseedweedseedweedseedweed";
            let private_key = BIP32ExtendedPrivateKey::from_seed(seed)
                .expect("cannot generate extended private key");
            let public_key = private_key
                .get_extended_public_key_legacy()
                .expect("cannot get extended public key");

            let private_child = private_key.private_child_legacy(1337);
            let private_grandchild = private_child.private_child_legacy(420);

            let public_child = public_key.public_child_legacy(1337);
            let public_grandchild = public_child.public_child_legacy(420);

            assert_eq!(
                public_grandchild,
                private_grandchild
                    .get_extended_public_key_legacy()
                    .expect("cannot get extended public key")
            );
        }
    }
}
