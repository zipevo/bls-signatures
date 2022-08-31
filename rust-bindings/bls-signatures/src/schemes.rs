use std::ffi::c_void;

use bls_dash_sys::{CAugSchemeMPLFree, CCoreMPLKeyGen, NewCAugSchemeMPL};

use crate::{private_key::PrivateKey, BlsError};

pub trait Scheme {
    fn as_mut_ptr(&self) -> *mut c_void;
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

//impl Scheme for AugSchemeMPL {}

impl Drop for AugSchemeMPL {
    fn drop(&mut self) {
        unsafe { CAugSchemeMPLFree(self.scheme) }
    }
}
