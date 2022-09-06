use std::ffi::c_void;

use bls_dash_sys::{
    CChainCodeFree, CChainCodeSerialize, CExtendedPublicKeyFree, CExtendedPublicKeyFromBytes,
    CExtendedPublicKeyGetChainCode, CExtendedPublicKeyPublicChild, CExtendedPublicKeySerialize,
};

use crate::{utils::c_err_to_result, BlsError};

pub const BIP32_EXTENDED_PUBLIC_KEY_SIZE: usize = 93;
pub const CHAIN_CODE_SIZE: usize = 32;

pub struct BIP32ExtendedPublicKey {
    c_extended_public_key: *mut c_void,
}

impl BIP32ExtendedPublicKey {
    fn from_bytes_with_legacy_flag(bytes: &[u8], legacy: bool) -> Result<Self, BlsError> {
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

    #[cfg(feature = "legacy")]
    pub fn from_bytes_legacy(bytes: &[u8]) -> Result<Self, BlsError> {
        Self::from_bytes_with_legacy_flag(bytes, true)
    }

    fn public_child_with_legacy_flag(&self, index: u32, legacy: bool) -> Self {
        BIP32ExtendedPublicKey {
            c_extended_public_key: unsafe {
                CExtendedPublicKeyPublicChild(self.c_extended_public_key, index, legacy)
            },
        }
    }

    pub fn public_child(&self, index: u32) -> Self {
        self.public_child_with_legacy_flag(index, false)
    }

    #[cfg(feature = "legacy")]
    pub fn public_child_legacy(&self, index: u32) -> Self {
        self.public_child_with_legacy_flag(index, true)
    }

    fn serialize_with_legacy_flag(
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

    #[cfg(feature = "legacy")]
    pub fn serialize_legacy(&self) -> Box<[u8; BIP32_EXTENDED_PUBLIC_KEY_SIZE]> {
        self.serialize_with_legacy_flag(true)
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

impl Drop for ChainCode {
    fn drop(&mut self) {
        unsafe { CChainCodeFree(self.c_chain_code) }
    }
}
