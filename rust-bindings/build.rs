use std::{env, fs};
use std::path::PathBuf;
use miette::IntoDiagnostic;

// fn main() {
//     // // Tell cargo to tell rustc to link the system bzip2
//     // // shared library.
//
//     cc::
//
//     println!("cargo:rustc-link-search=../build/_deps/sodium-build");
//     println!("cargo:rustc-link-lib=sodium");
//
//     println!("cargo:rustc-link-search=../build/_deps/relic-build/lib");
//     println!("cargo:rustc-link-lib=relic_s");
//
//     println!("cargo:rustc-link-search=/opt/homebrew/lib"); // TODO Get this from CMake too
//     println!("cargo:rustc-link-lib=gmp");
//
//     println!("cargo:rustc-link-search=../build/src");
//     println!("cargo:rustc-link-lib=bls-dash");
//
//     // The bindgen::Builder is the main entry point
//     // to bindgen, and lets you build up options for
//     // the resulting bindings.
//     let bindings = bindgen::Builder::default()
//         // The input header we would like to generate
//         // bindings for.
//         .header("wrapper.h")
//         // Tell cargo to invalidate the built crate whenever any of the
//         // included header files changed.
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
//         // Finish the builder and generate the bindings.
//         .generate()
//         // Unwrap the Result and panic on failure.
//         .expect("Unable to generate bindings");
//
//     // Write the bindings to the $OUT_DIR/bindings.rs file.
//     // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//     let out_path = PathBuf::from('.');
//     bindings
//         .write_to_file(out_path.join("bindings.rs"))
//         .expect("Couldn't write bindings!");
// }
//
//
fn main() -> miette::Result<()> {
    // TODO Build project
    // mkdir build
    // cd build
    // cmake ../
    // cmake --build . -- -j 6
    // Create build dir
    fs::create_dir_all("../build").into_diagnostic()?;


    // let include_paths = [
    //     std::path::PathBuf::from("../src"),
    //     std::path::PathBuf::from("../build/_deps/_deps/relic-src/include"),
    //     std::path::PathBuf::from("../build/_deps/relic-build/include"),
    //     std::path::PathBuf::from("../build/src"),
    //     std::path::PathBuf::from("/usr/include"),
    //     std::path::PathBuf::from("/opt/homebrew/opt/llvm/include"),
    //     // std::path::PathBuf::from("/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/"),
    //     std::path::PathBuf::from("/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/"),
    //     std::path::PathBuf::from("/opt/homebrew/include/"), // TODO gmp.h Use CMake generated files to get this path
    // ];
    //
    // let mut build = autocxx_build::Builder::new("src/bindings.rs", &include_paths)
    //     .extra_clang_args(&["-I/opt/homebrew/opt/llvm/include", "-I/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/"])
    //     // .build_listing_files()?;
    //     .build()?;
    //
    // build.flag_if_supported("-std=c++14")
    //     .archiver("/opt/homebrew/opt/llvm/bin/llvm-ar")
    //     .includes(&["/opt/homebrew/opt/llvm/include", "/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/"])
    //     // .warnings(false) // Disable all Clang warnings
    //     // .flag("-Wno-macro-redefined")// Should be disabled independently
    //     // .file("src/fake-chromium-src.cc") // If need to compile
    //     .compile("bls-signatures");


    // let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    // if target_arch.eq("wasm32") {
    // println!("cargo:rustc-cfg=feature=\"no-threads\"");
    // }

    // Rebuild of sources are changed
    // println!("cargo:rerun-if-changed=../src");
    // println!("cargo:rustc-flags=-I/opt/homebrew/opt/llvm/include");
    // println!("cargo:rustc-flags=-I/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/");
    //
    // println!("cargo:rustc-link-search=/opt/homebrew/opt/llvm/lib");
    // println!("cargo:rustc-link-search=/opt/homebrew/opt/llvm/include");
    // println!("cargo:rustc-link-search=/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/");
    // println!("cargo:rustc-env=AR=/opt/homebrew/opt/llvm/bin/llvm-ar");
    //
    // // Link BLS signatures library and dependencies
    //
    // println!("cargo:rustc-link-search=../build/_deps/sodium-build");
    // println!("cargo:rustc-link-lib=sodium");
    //
    // println!("cargo:rustc-link-search=../build/_deps/relic-build/lib");
    // println!("cargo:rustc-link-lib=relic_s");
    //
    // println!("cargo:rustc-link-search=/opt/homebrew/lib"); // TODO Get this from CMake too
    // println!("cargo:rustc-link-lib=gmp");
    //
    // println!("cargo:rustc-link-search=../build/src");
    // println!("cargo:rustc-link-lib=bls-dash");

    Ok(())
}