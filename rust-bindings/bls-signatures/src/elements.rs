use std::{ffi::c_void, marker::PhantomData};

use bls_dash_sys::{CG1ElementFree, CG1ElementFromBytes};

use crate::{utils::c_err_to_result, BlsError};

const G1_ELEMENT_SIZE: usize = 48; // TODO somehow extract it from bls library
const G2_ELEMENT_SIZE: usize = 96; // TODO somehow extract it from bls library

pub struct G1Element<'b> {
    pub(crate) element: *mut c_void,
    pub(crate) _bytes_lt: PhantomData<&'b ()>,
}

impl<'b> G1Element<'b> {
    pub fn from_bytes(bytes: &'b [u8]) -> Result<Self, BlsError> {
        if bytes.len() != G1_ELEMENT_SIZE {
            return Err(BlsError {
                msg: format!(
                    "G1 Element size must be {}, got {}",
                    G1_ELEMENT_SIZE,
                    bytes.len()
                ),
            });
        }
        Ok(G1Element {
            element: c_err_to_result(|did_err| unsafe {
                CG1ElementFromBytes(bytes.as_ptr() as *const _, did_err)
            })?,
            _bytes_lt: PhantomData,
        })
    }
}

impl Drop for G1Element<'_> {
    fn drop(&mut self) {
        unsafe { CG1ElementFree(self.element) }
    }
}

pub struct G2Element {
    element: *mut c_void,
}
