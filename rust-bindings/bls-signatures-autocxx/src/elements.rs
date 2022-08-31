use crate::bindings::BLS;
use crate::bindings::PrivateKey;
use crate::bindings::G1Element;
use crate::bindings::Util;
// use crate::bindings::Bytes;

use autocxx::prelude::*;

pub struct G1Element2 {
    // internal: CG1Element

}

impl G1Element2 {
    pub fn do_something() {
        BLS::CheckRelicErrors();
    }
}

//
// impl From<Vec<u8>> for G1Element {
//     fn from(bytes: Vec<u8>) -> Self {
//     }
// }

#[cfg(test)]
mod tests {
    use std::os::raw::c_char;
    use autocxx::moveit::MakeCppStorage;
    use super::*;

    #[test]
    fn test() {
        let a: Vec<u8> = vec![1, 2, 3, 4];

        // let bytes = unsafe {
        //     let a = Bytes::allocate_uninitialized_cpp_storage();
        //     a.
        //     a.as_ref()
        // };

        let element = moveit! {
            G1Element::FromByteVector(a.as_mut_ptr(), true)
        };

        moveit! { let mut msg = PrivateKey::RandomPrivateKey(); }

        // let mut buf = Util;

        // unsafe {
        //     msg.Serialize(&mut buf);
        // };
        //
        // println!("{}", buf);
        println!("Hello!");


        // let vec: CxxVector<u8> = a.try_into().unwrap();

        // let element = G1Element::FromByteVector(&vec, true);

        BLS::CheckRelicErrors();
    }

    mod from {
        use super::*;

        #[test]
        fn should_create_from_bytes() {
            // G1Element::from();
        }
    }
}