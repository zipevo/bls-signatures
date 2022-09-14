use bls_dash_sys as sys;

#[test]
fn sign_and_verify() {
    let seed = b"seedweedseedweedseedweedseedweed";
    let bad_seed = b"weedseedweedseedweedseedweedseed";

    unsafe {
        let scheme = sys::NewAugSchemeMPL();
        let mut did_err = false;

        let sk = sys::CoreMPLKeyGen(
            scheme,
            seed.as_ptr() as *const _,
            seed.len(),
            &mut did_err as *mut _,
        );
        assert!(!did_err);

        let pk = sys::PrivateKeyGetG1Element(sk, &mut did_err as *mut _);
        assert!(!did_err);

        let sk2 = sys::CoreMPLKeyGen(
            scheme,
            bad_seed.as_ptr() as *const _,
            bad_seed.len(),
            &mut did_err as *mut _,
        );
        assert!(!did_err);

        let pk2 = sys::PrivateKeyGetG1Element(sk2, &mut did_err as *mut _);
        assert!(!did_err);

        let message = b"Evgeny owns 1337 dash no cap";
        let sig = sys::CoreMPLSign(scheme, sk, message.as_ptr() as *const _, message.len());

        let verify =
            sys::CoreMPLVerify(scheme, pk, message.as_ptr() as *const _, message.len(), sig);
        assert!(verify);

        let verify_bad = sys::CoreMPLVerify(
            scheme,
            pk2,
            message.as_ptr() as *const _,
            message.len(),
            sig,
        );
        assert!(!verify_bad);

        sys::G2ElementFree(sig);
        sys::G1ElementFree(pk2);
        sys::PrivateKeyFree(sk2);
        sys::G1ElementFree(pk);
        sys::PrivateKeyFree(sk);
        sys::AugSchemeMPLFree(scheme);
    }
}
