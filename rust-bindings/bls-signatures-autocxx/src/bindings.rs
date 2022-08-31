use autocxx::prelude::*;

include_cpp! {
    #include "bls.hpp"
    #include "elements.hpp"
    #include "privatekey.hpp"
    #include "extendedprivatekey.hpp"
    #include "schemes.hpp"
    #include "threshold.hpp"
    #include "util.hpp"

    safety!(unsafe)

    // bls
    generate!("bls::BLS")

    // privatekey
    generate!("bls::PrivateKey")

    // extendedprivatekey
    generate!("bls::ExtendedPrivateKey")

    // elements
    generate!("bls::G1Element")
    generate!("bls::G2Element")

    // schemes
    generate!("bls::CoreMPL")
    generate!("bls::BasicSchemeMPL")
    generate!("bls::AugSchemeMPL")
    generate!("bls::PopSchemeMPL")
    generate!("bls::LegacySchemeMPL")

    // threshold
    generate_ns!("bls::Threshold")

    // util
    generate!("bls::Util")
}

pub use ffi::bls::*;
