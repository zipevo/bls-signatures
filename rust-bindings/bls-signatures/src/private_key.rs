use std::ffi::c_void;
use std::ops::Mul;

use bls_dash_sys::{
    CoreMPLDeriveChildSk, CoreMPLDeriveChildSkUnhardened, CoreMPLKeyGen, G1ElementMul,
    PrivateKeyFree, PrivateKeyFromBytes, PrivateKeyFromSeedBIP32, PrivateKeyGetG1Element,
    PrivateKeyIsEqual, PrivateKeySerialize,
};

use crate::{
    schemes::Scheme,
    utils::{c_err_to_result, SecureBox},
    BlsError, G1Element,
};

pub const PRIVATE_KEY_SIZE: usize = 32; // TODO somehow extract it from bls library

#[derive(Debug)]
pub struct PrivateKey {
    pub(crate) c_private_key: *mut c_void,
}

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        unsafe { PrivateKeyIsEqual(self.c_private_key, other.c_private_key) }
    }
}

impl Eq for PrivateKey {}

impl Mul<G1Element> for PrivateKey {
    type Output = Result<G1Element, BlsError>;

    fn mul(self, rhs: G1Element) -> Self::Output {
        Ok(G1Element {
            c_element: c_err_to_result(|did_err| unsafe {
                G1ElementMul(rhs.c_element, self.c_private_key)
            })?,
        })
    }
}

impl Mul<PrivateKey> for G1Element {
    type Output = Result<G1Element, BlsError>;

    fn mul(self, rhs: PrivateKey) -> Self::Output {
        rhs * self
    }
}

impl PrivateKey {
    pub(crate) fn as_mut_ptr(&self) -> *mut c_void {
        self.c_private_key
    }

    // TODO Rename to from_seed
    pub fn key_gen(scheme: &impl Scheme, seed: &[u8]) -> Result<Self, BlsError> {
        Ok(PrivateKey {
            c_private_key: c_err_to_result(|did_err| unsafe {
                CoreMPLKeyGen(
                    scheme.as_mut_ptr(),
                    seed.as_ptr() as *const _,
                    seed.len(),
                    did_err,
                )
            })?,
        })
    }

    pub fn g1_element(&self) -> Result<G1Element, BlsError> {
        Ok(G1Element {
            c_element: c_err_to_result(|did_err| unsafe {
                PrivateKeyGetG1Element(self.c_private_key, did_err)
            })?,
        })
    }

    pub fn serialize(&self) -> SecureBox {
        // `PrivateKeySerialize` internally securely allocates memory which we have to
        // wrap safely
        unsafe {
            SecureBox::from_ptr(
                PrivateKeySerialize(self.c_private_key) as *mut u8,
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
            PrivateKeyFromBytes(bytes.as_ptr() as *const c_void, mod_order, did_err)
        })?;

        Ok(PrivateKey { c_private_key })
    }

    pub fn from_bip32_seed(bytes: &[u8]) -> Self {
        let c_private_key = unsafe { PrivateKeyFromSeedBIP32(bytes.as_ptr() as *const c_void) };

        PrivateKey { c_private_key }
    }

    pub fn derive_child_private_key(&self, scheme: &impl Scheme, index: u32) -> PrivateKey {
        PrivateKey {
            c_private_key: unsafe {
                CoreMPLDeriveChildSk(scheme.as_mut_ptr(), self.c_private_key, index)
            },
        }
    }

    pub fn derive_child_private_key_unhardened(
        &self,
        scheme: &impl Scheme,
        index: u32,
    ) -> PrivateKey {
        PrivateKey {
            c_private_key: unsafe {
                CoreMPLDeriveChildSkUnhardened(scheme.as_mut_ptr(), self.c_private_key, index)
            },
        }
    }
}

impl Drop for PrivateKey {
    fn drop(&mut self) {
        unsafe { PrivateKeyFree(self.c_private_key) }
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

    #[test]
    fn should_return_private_key_from_bip32_bytes() {
        let bytes = [1, 2, 3, 4, 5, 6, 7];
        let private_key = PrivateKey::from_bip32_seed(&bytes);
        let expected_key_bytes = [
            0, 40, 43, 250, 83, 117, 227, 93, 174, 67, 170, 185, 235, 46, 70, 117, 110, 208, 224,
            23, 164, 13, 180, 200, 132, 46, 57, 21, 207, 149, 248, 135,
        ];
        assert_eq!(*private_key.serialize(), expected_key_bytes);
        let alice_seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let pk1 = PrivateKey::from_bip32_seed(&bytes);
        let private_key = pk1.serialize();
    }

    #[test]
    fn test_keys_multiplication() {
        //46891c2cec49593c81921e473db7480029e0fc1eb933c6b93d81f5370eb19fbd
        let private_key_data = [
            70, 137, 28, 44, 236, 73, 89, 60, 129, 146, 30, 71, 61, 183, 72, 0, 41, 224, 252, 30,
            185, 51, 198, 185, 61, 129, 245, 55, 14, 177, 159, 189,
        ];
        //0e2f9055c17eb13221d8b41833468ab49f7d4e874ddf4b217f5126392a608fd48ccab3510548f1da4f397c1ad4f8e01a
        let public_key_data = [
            14, 47, 144, 85, 193, 126, 177, 50, 33, 216, 180, 24, 51, 70, 138, 180, 159, 125, 78,
            135, 77, 223, 75, 33, 127, 81, 38, 57, 42, 96, 143, 212, 140, 202, 179, 81, 5, 72, 241,
            218, 79, 57, 124, 26, 212, 248, 224, 26,
        ];
        //03fd387c4d4c66ec9dcdb31ef0c08ad881090dcda13d4b2c9cbc5ef264ff4dc7
        let expected_data = [
            3, 253, 56, 124, 77, 76, 102, 236, 157, 205, 179, 30, 240, 192, 138, 216, 129, 9, 13,
            205, 161, 61, 75, 44, 156, 188, 94, 242, 100, 255, 77, 199,
        ];
        let private_key = PrivateKey::from_bytes(&private_key_data, false).unwrap();
        let public_key = G1Element::from_bytes_legacy(&public_key_data).unwrap();
        let result = (private_key * public_key).unwrap();
        assert_eq!(
            &result.serialize_legacy()[..32],
            &expected_data,
            "should match"
        );
        let private_key = PrivateKey::from_bytes(&private_key_data, false).unwrap();
        let public_key = G1Element::from_bytes_legacy(&public_key_data).unwrap();
        let result = (public_key * private_key).unwrap();
        assert_eq!(
            &result.serialize_legacy()[..32],
            &expected_data,
            "should match"
        );
    }
}
