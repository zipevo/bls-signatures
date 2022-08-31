use std::ffi::c_void;

pub struct G1Element {
    element: *mut c_void,
}

pub struct G2Element {
    element: *mut c_void,
}
