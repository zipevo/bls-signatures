use bls_dash_sys as sys;

#[test]
fn bls_dash_linked_sign_verify_simple() {
    let seed = b"seedweedseedweedseedweedseedweed";
    let bad_seed = b"weedseedweedseedweedseedweedseed";

    unsafe {
        let scheme = sys::NewCAugSchemeMPL();
        let mut did_err = false;

        let sk = sys::CCoreMPLKeyGen(
            scheme,
            seed.as_ptr() as *const _,
            seed.len(),
            &mut did_err as *mut _,
        );
        assert!(!did_err);

        let pk = sys::CPrivateKeyGetG1Element(sk, &mut did_err as *mut _);
        assert!(!did_err);

        let sk2 = sys::CCoreMPLKeyGen(
            scheme,
            bad_seed.as_ptr() as *const _,
            bad_seed.len(),
            &mut did_err as *mut _,
        );
        assert!(!did_err);

        let pk2 = sys::CPrivateKeyGetG1Element(sk2, &mut did_err as *mut _);
        assert!(!did_err);

        let message = b"Evgeny owns 1337 dash no cap";
        let sig = sys::CCoreMPLSign(scheme, sk, message.as_ptr() as *const _, message.len());

        let verify =
            sys::CCoreMPLVerify(scheme, pk, message.as_ptr() as *const _, message.len(), sig);
        assert!(verify);

        let verify_bad = sys::CCoreMPLVerify(
            scheme,
            pk2,
            message.as_ptr() as *const _,
            message.len(),
            sig,
        );
        assert!(!verify_bad);

        sys::CG2ElementFree(sig);
        sys::CG1ElementFree(pk2);
        sys::CPrivateKeyFree(sk2);
        sys::CG1ElementFree(pk);
        sys::CPrivateKeyFree(sk);
        sys::CAugSchemeMPLFree(scheme);
    }
}
