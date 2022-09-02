use std::{ffi::c_void, marker::PhantomData};

use bls_dash_sys::{
    CCoreMPLKeyGen, CPrivateKeyFree, CPrivateKeyFromBytes, CPrivateKeyGetG1Element,
    CPrivateKeyIsEqual, CPrivateKeySerialize,
};

use crate::{
    schemes::Scheme,
    utils::{c_err_to_result, SecAlloc},
    BlsError, G1Element,
};

const PRIVATE_KEY_SIZE: usize = 32; // TODO somehow extract it from bls library

#[derive(Debug)]
pub struct PrivateKey {
    c_private_key: *mut c_void,
}

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CPrivateKeyIsEqual(self.c_private_key, other.c_private_key) }
    }
}

impl PrivateKey {
    pub(crate) fn as_mut_ptr(&self) -> *mut c_void {
        self.c_private_key
    }

    pub fn key_gen(scheme: &impl Scheme, seed: &[u8]) -> Result<Self, BlsError> {
        Ok(PrivateKey {
            c_private_key: c_err_to_result(|did_err| unsafe {
                CCoreMPLKeyGen(
                    scheme.as_mut_ptr(),
                    seed.as_ptr() as *const _,
                    seed.len(),
                    did_err,
                )
            })?,
        })
    }

    pub fn get_g1_element<'a>(&'a self) -> Result<G1Element<'a>, BlsError> {
        Ok(G1Element {
            element: c_err_to_result(|did_err| unsafe {
                CPrivateKeyGetG1Element(self.c_private_key, did_err)
            })?,
            _bytes_lt: PhantomData,
        })
    }

    pub fn serialize(&self) -> SecAlloc {
        // `CPrivateKeySerialize` internally securely allocates memory which we have to
        // wrap safely
        unsafe {
            SecAlloc::from_ptr(
                CPrivateKeySerialize(self.c_private_key) as *mut u8,
                PRIVATE_KEY_SIZE,
            )
        }
    }

    pub fn from_bytes(bytes: &[u8], mod_order: bool) -> Result<Self, BlsError> {
        if bytes.len() != PRIVATE_KEY_SIZE {
            return Err(BlsError {
                msg: format!(
                    "Private key size must be {}, got {}",
                    PRIVATE_KEY_SIZE,
                    bytes.len()
                ),
            });
        }

        let c_private_key = c_err_to_result(|did_err| unsafe {
            CPrivateKeyFromBytes(bytes.as_ptr() as *const c_void, mod_order, did_err)
        })?;
        Ok(PrivateKey { c_private_key })
    }
}

impl Drop for PrivateKey {
    fn drop(&mut self) {
        unsafe { CPrivateKeyFree(self.c_private_key) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemes::AugSchemeMPL;

    #[test]
    fn serialize_deserialize() {
        let seed = b"seedweedseedweedseedweedseedweed";
        let scheme = AugSchemeMPL::new();
        let sk1 = PrivateKey::key_gen(&scheme, seed).expect("unable to generate private key");
        let sk1_bytes = sk1.serialize();
        let sk2 = PrivateKey::from_bytes(sk1_bytes.as_slice(), false)
            .expect("cannot build private key from bytes");

        assert_eq!(sk1, sk2);
    }
}
