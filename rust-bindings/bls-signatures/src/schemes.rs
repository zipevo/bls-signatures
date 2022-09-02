use std::ffi::c_void;

use bls_dash_sys::{
    CAugSchemeMPLFree, CAugSchemeMPLSign, CAugSchemeMPLVerify, CCoreMPLKeyGen, NewCAugSchemeMPL,
};

use crate::{private_key::PrivateKey, BlsError, G1Element, G2Element};

pub trait Scheme {
    fn as_mut_ptr(&self) -> *mut c_void;

    fn sign(&self, private_key: &PrivateKey, message: &[u8]) -> G2Element;

    fn verify(&self, public_key: &G1Element, message: &[u8], signature: &G2Element) -> bool;
}

pub struct AugSchemeMPL {
    scheme: *mut c_void,
}

impl AugSchemeMPL {
    pub fn new() -> Self {
        AugSchemeMPL {
            scheme: unsafe { NewCAugSchemeMPL() },
        }
    }
}

impl Scheme for AugSchemeMPL {
    fn as_mut_ptr(&self) -> *mut c_void {
        self.scheme
    }

    fn sign(&self, private_key: &PrivateKey, message: &[u8]) -> G2Element {
        G2Element {
            element: unsafe {
                CAugSchemeMPLSign(
                    self.scheme,
                    private_key.as_mut_ptr(),
                    message.as_ptr() as *const _,
                    message.len(),
                )
            },
        }
    }

    fn verify(&self, public_key: &G1Element, message: &[u8], signature: &G2Element) -> bool {
        unsafe {
            CAugSchemeMPLVerify(
                self.scheme,
                public_key.element,
                message.as_ptr() as *const _,
                message.len(),
                signature.element,
            )
        }
    }
}

impl Drop for AugSchemeMPL {
    fn drop(&mut self) {
        unsafe { CAugSchemeMPLFree(self.scheme) }
    }
}
