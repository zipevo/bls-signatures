use std::{env, fs};
use std::path::{Path, PathBuf};

fn abs(path: &str) -> String {
    let path_buf = PathBuf::from(path);

    let path_abs = fs::canonicalize(&path_buf).expect("should provide valid abs path");

    path_abs.to_str().expect("should convert path to string").to_owned()
}

fn main() {
    // let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    // if target_arch.eq("wasm32") {
    // }

    // TODO Run cmake or emcmake depends on target
    // mkdir build
    // cd build
    // cmake ../
    // cmake --build . -- -j 6
    // Create build dir
    // fs::create_dir_all("../build").unwrap();

    // // Tell cargo to tell rustc to link the system bzip2
    // // shared library.

    let library_path = Path::new("../go-bindings");

    cc::Build::new()
        .files(["../go-bindings/blschia.cpp"])
        .include(library_path)
        .cpp(true)
        .define("_LIBCPP_HAS_NO_THREADS", Some("1"))
        .flag_if_supported("-std=c++14")
        .includes([
                      abs("../build/_deps/relic-src/include").as_str(),
                      abs("../build/_deps/relic-build/include").as_str(),
                      abs("../build/src").as_str(),
                      "/opt/homebrew/opt/llvm/include",
                      "/usr/include",

            // "/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/",
            // "/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/",

            "/opt/homebrew/Cellar/emscripten/3.1.19/libexec/cache/sysroot/include/c++/v1/",
            "/opt/homebrew/Cellar/emscripten/3.1.19/libexec/cache/sysroot/include/",


            "/opt/homebrew/include/",
        ])
        .include("../src")
        .compile("go-binding");

    println!("cargo:rustc-link-search={}", abs("../build/_deps/sodium-build"));
    println!("cargo:rustc-link-lib=sodium");

    println!("cargo:rustc-link-search={}", abs("../build/_deps/relic-build/lib"));
    println!("cargo:rustc-link-lib=relic_s");

    println!("cargo:rustc-link-search=/opt/homebrew/lib");
    println!("cargo:rustc-link-lib=gmp");

    println!("cargo:rustc-link-search={}", abs("../build/src"));
    println!("cargo:rustc-link-lib=bls-dash");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .detect_include_paths(true)
        .dynamic_link_require_all(true)
        // .clang_arg("-I../src")
        // .clang_arg("-std=c++14")
        .clang_arg("-I../build/_deps/relic-src/include")
        .clang_arg("-I../build/_deps/relic-build/include")
        .clang_arg("-I../build/src")
        .clang_arg("-I/opt/homebrew/opt/llvm/include")
        .clang_arg("-I/usr/include")

        // .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/")
        // .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/")

        .clang_arg("-I/opt/homebrew/Cellar/emscripten/3.1.19/libexec/cache/sysroot/include/c++/v1/")
        .clang_arg("-I/opt/homebrew/Cellar/emscripten/3.1.19/libexec/cache/sysroot/include/")

        .clang_arg("-I/opt/homebrew/include/")


        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}