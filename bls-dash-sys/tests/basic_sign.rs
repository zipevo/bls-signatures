use bls_dash_sys::bindings;

#[test]
fn bls_dash_linked_sign_verify_simple() {
    let seed = b"seedweedseedweedseedweedseedweed";
    let bad_seed = b"weedseedweedseedweedseedweedseed";

    unsafe {
        let scheme = bindings::NewCAugSchemeMPL();
        let mut did_err = false;

        let sk = bindings::CCoreMPLKeyGen(
            scheme,
            seed.as_ptr() as *const _,
            seed.len(),
            &mut did_err as *mut _,
        );
        assert!(!did_err);

        let pk = bindings::CPrivateKeyGetG1Element(sk, &mut did_err as *mut _);
        assert!(!did_err);

        let sk2 = bindings::CCoreMPLKeyGen(
            scheme,
            bad_seed.as_ptr() as *const _,
            bad_seed.len(),
            &mut did_err as *mut _,
        );
        assert!(!did_err);

        let pk2 = bindings::CPrivateKeyGetG1Element(sk2, &mut did_err as *mut _);
        assert!(!did_err);

        let message = b"Evgeny owns 1337 dash no cap";
        let sig = bindings::CCoreMPLSign(scheme, sk, message.as_ptr() as *const _, message.len());

        let verify =
            bindings::CCoreMPLVerify(scheme, pk, message.as_ptr() as *const _, message.len(), sig);
        assert!(verify);

        let verify_bad = bindings::CCoreMPLVerify(
            scheme,
            pk2,
            message.as_ptr() as *const _,
            message.len(),
            sig,
        );
        assert!(!verify_bad);

        bindings::CG2ElementFree(sig);
        bindings::CG1ElementFree(pk2);
        bindings::CPrivateKeyFree(sk2);
        bindings::CG1ElementFree(pk);
        bindings::CPrivateKeyFree(sk);
        bindings::CAugSchemeMPLFree(scheme);
    }
}
