#[cfg(test)]
mod tests {
    use bls_dash_sys::bindings;

    #[test]
    fn it_works_somehow() {
        let seed = b"seedweedseedweedseedweedseedweed";
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

            let message = b"Evgeny owns 1337 dash no cap";
            let sig =
                bindings::CCoreMPLSign(scheme, sk, message.as_ptr() as *const _, message.len());

            let verify = bindings::CCoreMPLVerify(
                scheme,
                pk,
                message.as_ptr() as *const _,
                message.len(),
                sig,
            );
            assert!(verify);
        }
    }
}
