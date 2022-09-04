use std::ffi::c_void;

use bls_dash_sys::{
    CAugSchemeMPLAggregateVerify, CAugSchemeMPLFree, CAugSchemeMPLSign, CAugSchemeMPLVerify,
    CBasicSchemeMPLAggregateVerify, CBasicSchemeMPLFree, CCoreMPLAggregatePubKeys,
    CCoreMPLAggregateSigs, CCoreMPLAggregateVerify, CCoreMPLKeyGen, CCoreMPLSign, CCoreMPLVerify,
    NewCAugSchemeMPL, NewCBasicSchemeMPL,
};

use crate::{private_key::PrivateKey, BlsError, G1Element, G2Element};

pub trait Scheme {
    fn as_mut_ptr(&self) -> *mut c_void;

    fn sign(&self, private_key: &PrivateKey, message: &[u8]) -> G2Element;

    fn verify(&self, public_key: &G1Element, message: &[u8], signature: &G2Element) -> bool;

    fn aggregate_public_keys<'a>(
        &self,
        public_keys: impl IntoIterator<Item = &'a G1Element>,
    ) -> G1Element {
        let mut g1_pointers = public_keys
            .into_iter()
            .map(|g1| g1.c_element)
            .collect::<Vec<_>>();
        G1Element {
            c_element: unsafe {
                CCoreMPLAggregatePubKeys(
                    self.as_mut_ptr(),
                    g1_pointers.as_mut_ptr(),
                    g1_pointers.len(),
                )
            },
        }
    }

    fn aggregate_sigs<'a>(&self, sigs: impl IntoIterator<Item = &'a G2Element>) -> G2Element {
        let mut g2_pointers = sigs.into_iter().map(|g2| g2.c_element).collect::<Vec<_>>();
        G2Element {
            c_element: unsafe {
                CCoreMPLAggregateSigs(
                    self.as_mut_ptr(),
                    g2_pointers.as_mut_ptr(),
                    g2_pointers.len(),
                )
            },
        }
    }

    fn aggregate_verify<'a>(
        &self,
        public_keys: impl IntoIterator<Item = &'a G1Element>,
        messages: impl IntoIterator<Item = &'a [u8]>,
        signature: &G2Element,
    ) -> bool;
}

struct AggregateVerifyArgs {
    g1_pointers: Vec<*mut c_void>,
    messages_pointers: Vec<*const u8>,
    messages_lengthes: Vec<usize>,
}

fn prepare_aggregate_verify_args<'a>(
    public_keys: impl IntoIterator<Item = &'a G1Element>,
    messages: impl IntoIterator<Item = &'a [u8]>,
) -> AggregateVerifyArgs {
    let g1_pointers = public_keys
        .into_iter()
        .map(|g1| g1.c_element)
        .collect::<Vec<_>>();

    let mut messages_pointers = Vec::new();
    let mut messages_lengthes = Vec::new();

    for m in messages.into_iter() {
        messages_pointers.push(m.as_ptr());
        messages_lengthes.push(m.len());
    }

    AggregateVerifyArgs {
        g1_pointers,
        messages_pointers,
        messages_lengthes,
    }
}

pub struct BasicSchemeMPL {
    scheme: *mut c_void,
}

impl BasicSchemeMPL {
    pub fn new() -> Self {
        BasicSchemeMPL {
            scheme: unsafe { NewCBasicSchemeMPL() },
        }
    }
}

impl Scheme for BasicSchemeMPL {
    fn as_mut_ptr(&self) -> *mut c_void {
        self.scheme
    }

    fn sign(&self, private_key: &PrivateKey, message: &[u8]) -> G2Element {
        G2Element {
            c_element: unsafe {
                CCoreMPLSign(
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
            CCoreMPLVerify(
                self.scheme,
                public_key.c_element,
                message.as_ptr() as *const _,
                message.len(),
                signature.c_element,
            )
        }
    }

    fn aggregate_verify<'a>(
        &self,
        public_keys: impl IntoIterator<Item = &'a G1Element>,
        messages: impl IntoIterator<Item = &'a [u8]>,
        signature: &G2Element,
    ) -> bool {
        let AggregateVerifyArgs {
            mut g1_pointers,
            mut messages_pointers,
            mut messages_lengthes,
        } = prepare_aggregate_verify_args(public_keys, messages);

        unsafe {
            CBasicSchemeMPLAggregateVerify(
                self.as_mut_ptr(),
                g1_pointers.as_mut_ptr(),
                g1_pointers.len(),
                messages_pointers.as_mut_ptr() as *mut _,
                messages_lengthes.as_mut_ptr() as *mut _,
                messages_pointers.len(),
                signature.c_element,
            )
        }
    }
}

impl Drop for BasicSchemeMPL {
    fn drop(&mut self) {
        unsafe { CBasicSchemeMPLFree(self.scheme) }
    }
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
            c_element: unsafe {
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
                public_key.c_element,
                message.as_ptr() as *const _,
                message.len(),
                signature.c_element,
            )
        }
    }

    fn aggregate_verify<'a>(
        &self,
        public_keys: impl IntoIterator<Item = &'a G1Element>,
        messages: impl IntoIterator<Item = &'a [u8]>,
        signature: &G2Element,
    ) -> bool {
        let AggregateVerifyArgs {
            mut g1_pointers,
            mut messages_pointers,
            mut messages_lengthes,
        } = prepare_aggregate_verify_args(public_keys, messages);

        unsafe {
            CAugSchemeMPLAggregateVerify(
                self.as_mut_ptr(),
                g1_pointers.as_mut_ptr(),
                g1_pointers.len(),
                messages_pointers.as_mut_ptr() as *mut _,
                messages_lengthes.as_mut_ptr() as *mut _,
                messages_pointers.len(),
                signature.c_element,
            )
        }
    }
}

impl Drop for AugSchemeMPL {
    fn drop(&mut self) {
        unsafe { CAugSchemeMPLFree(self.scheme) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_aggregate() {
        let seed1 = b"seedweedseedweedseedweedseedweed";
        let seed2 = b"weedseedweedseedweedseedweedseed";
        let seed3 = b"seedseedseedseedweedweedweedweed";

        let scheme = AugSchemeMPL::new();

        let private_key_1 =
            PrivateKey::key_gen(&scheme, seed1).expect("unable to generate private key");
        let private_key_2 =
            PrivateKey::key_gen(&scheme, seed2).expect("unable to generate private key");
        let private_key_3 =
            PrivateKey::key_gen(&scheme, seed3).expect("unable to generate private key");

        let public_key_1 = private_key_1
            .get_g1_element()
            .expect("unable to get public key");
        let public_key_2 = private_key_2
            .get_g1_element()
            .expect("unable to get public key");
        let public_key_3 = private_key_3
            .get_g1_element()
            .expect("unable to get public key");

        let message_1 = b"ayya";
        let message_2 = b"ayyb";
        let message_3 = b"ayyc";

        let signature_1 = scheme.sign(&private_key_1, message_1);
        let signature_2 = scheme.sign(&private_key_2, message_2);
        let signature_3 = scheme.sign(&private_key_3, message_3);

        let signature_agg = scheme.aggregate_sigs([&signature_1, &signature_2, &signature_3]);

        let verify = scheme.aggregate_verify(
            [&public_key_1, &public_key_2, &public_key_3],
            [message_1.as_ref(), message_2.as_ref(), message_3.as_ref()],
            &signature_agg,
        );
        assert!(verify);
    }
}
