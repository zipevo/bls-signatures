use std::{ffi::c_void, marker::PhantomData};

use bls_dash_sys::{CCoreMPLKeyGen, CPrivateKeyFree, CPrivateKeyGetG1Element};

use crate::{schemes::Scheme, utils::c_err_to_result, BlsError, G1Element};

pub struct PrivateKey {
    private_key: *mut c_void,
}

impl PrivateKey {
    pub fn key_gen(scheme: impl Scheme, seed: &[u8]) -> Result<Self, BlsError> {
        Ok(PrivateKey {
            private_key: c_err_to_result(|did_err| unsafe {
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
                CPrivateKeyGetG1Element(self.private_key, did_err)
            })?,
            _bytes_lt: PhantomData,
        })
    }
}

impl Drop for PrivateKey {
    fn drop(&mut self) {
        unsafe { CPrivateKeyFree(self.private_key) }
    }
}
