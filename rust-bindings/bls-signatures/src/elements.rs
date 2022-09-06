use std::ffi::c_void;

use bls_dash_sys::{
    CCoreMPLDeriveChildPkUnhardened, CG1ElementFree, CG1ElementFromBytes, CG1ElementIsEqual,
    CG1ElementSerialize, CG2ElementFree, CG2ElementFromBytes, CG2ElementIsEqual,
    CG2ElementSerialize,
};

use crate::{schemes::Scheme, utils::c_err_to_result, BlsError};

pub const G1_ELEMENT_SIZE: usize = 48; // TODO somehow extract it from bls library
pub const G2_ELEMENT_SIZE: usize = 96; // TODO somehow extract it from bls library

#[derive(Debug)]
pub struct G1Element {
    pub(crate) c_element: *mut c_void,
}

impl PartialEq for G1Element {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CG1ElementIsEqual(self.c_element, other.c_element) }
    }
}

impl Eq for G1Element {}

impl G1Element {
    fn from_bytes_with_legacy_flag(bytes: &[u8], legacy: bool) -> Result<Self, BlsError> {
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
            c_element: c_err_to_result(|did_err| unsafe {
                CG1ElementFromBytes(bytes.as_ptr() as *const _, legacy, did_err)
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

    fn serialize_with_legacy_flag(&self, legacy: bool) -> Box<[u8; G1_ELEMENT_SIZE]> {
        unsafe {
            let malloc_ptr = CG1ElementSerialize(self.c_element, legacy);
            Box::from_raw(malloc_ptr as *mut _)
        }
    }

    pub fn serialize(&self) -> Box<[u8; G1_ELEMENT_SIZE]> {
        self.serialize_with_legacy_flag(false)
    }

    #[cfg(feature = "legacy")]
    pub fn serialize_legacy(&self) -> Box<[u8; G1_ELEMENT_SIZE]> {
        self.serialize_with_legacy_flag(true)
    }

    pub fn derive_child_public_key_unhardened(
        &self,
        scheme: &impl Scheme,
        index: u32,
    ) -> G1Element {
        G1Element {
            c_element: unsafe {
                CCoreMPLDeriveChildPkUnhardened(scheme.as_mut_ptr(), self.c_element, index)
            },
        }
    }
}

impl Drop for G1Element {
    fn drop(&mut self) {
        unsafe { CG1ElementFree(self.c_element) }
    }
}

#[derive(Debug)]
pub struct G2Element {
    pub(crate) c_element: *mut c_void,
}

impl PartialEq for G2Element {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CG2ElementIsEqual(self.c_element, other.c_element) }
    }
}

impl Eq for G2Element {}

impl G2Element {
    pub fn from_bytes_with_legacy_flag(bytes: &[u8], legacy: bool) -> Result<Self, BlsError> {
        if bytes.len() != G2_ELEMENT_SIZE {
            return Err(BlsError {
                msg: format!(
                    "G2 Element size must be {}, got {}",
                    G2_ELEMENT_SIZE,
                    bytes.len()
                ),
            });
        }
        Ok(G2Element {
            c_element: c_err_to_result(|did_err| unsafe {
                CG2ElementFromBytes(bytes.as_ptr() as *const _, legacy, did_err)
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

    pub fn serialize_with_legacy_flag(&self, legacy: bool) -> Box<[u8; G2_ELEMENT_SIZE]> {
        unsafe {
            let malloc_ptr = CG2ElementSerialize(self.c_element, legacy);
            Box::from_raw(malloc_ptr as *mut _)
        }
    }

    pub fn serialize(&self) -> Box<[u8; G2_ELEMENT_SIZE]> {
        self.serialize_with_legacy_flag(false)
    }

    #[cfg(feature = "legacy")]
    pub fn serialize_legacy(&self) -> Box<[u8; G2_ELEMENT_SIZE]> {
        self.serialize_with_legacy_flag(true)
    }
}

impl Drop for G2Element {
    fn drop(&mut self) {
        unsafe { CG2ElementFree(self.c_element) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        schemes::{AugSchemeMPL, Scheme},
        PrivateKey,
    };

    #[test]
    fn g1_serialize_deserialize() {
        let seed = b"seedweedseedweedseedweedseedweed";
        let scheme = AugSchemeMPL::new();
        let sk = PrivateKey::key_gen(&scheme, seed).expect("unable to generate private key");

        let g1 = sk.get_g1_element().expect("cannot get G1 element");
        let g1_bytes = g1.serialize();
        let g1_2 =
            G1Element::from_bytes(g1_bytes.as_ref()).expect("cannot build G1 element from bytes");

        assert_eq!(g1, g1_2);
    }

    #[test]
    fn g2_serialize_deserialize() {
        let seed = b"seedweedseedweedseedweedseedweed";
        let scheme = AugSchemeMPL::new();
        let sk = PrivateKey::key_gen(&scheme, seed).expect("unable to generate private key");

        let g2 = scheme.sign(&sk, b"ayy");
        let g2_bytes = g2.serialize();
        let g2_2 =
            G2Element::from_bytes(g2_bytes.as_ref()).expect("cannot build G2 element from bytes");

        assert_eq!(g2, g2_2);
    }
}
