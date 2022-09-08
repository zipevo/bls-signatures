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

pub const BIP32_CHAIN_CODE_SIZE: usize = 32;

#[derive(Debug)]
pub struct ChainCode {
    pub(crate) c_chain_code: *mut c_void,
}

impl ChainCode {
    pub fn serialize(&self) -> Box<[u8; BIP32_CHAIN_CODE_SIZE]> {
        unsafe {
            let malloc_ptr = CBIP32ChainCodeSerialize(self.c_chain_code);
            Box::from_raw(malloc_ptr as *mut _)
        }
    }
}

impl PartialEq for ChainCode {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CBIP32ChainCodeIsEqual(self.c_chain_code, other.c_chain_code) }
    }
}

impl Eq for ChainCode {}

impl Drop for ChainCode {
    fn drop(&mut self) {
        unsafe { CBIP32ChainCodeFree(self.c_chain_code) }
    }
}
