use std::{env, fs};
use std::path::{Path, PathBuf};
use miette::IntoDiagnostic;

fn abs(path: &str) -> String {
    let path_buf = PathBuf::from(path);

    let path_abs = fs::canonicalize(&path_buf).expect("should provide valid abs path");

    path_abs.to_str().expect("should convert path to string").to_owned()
}

fn main() -> miette::Result<()> {
    // let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    // if target_arch.eq("wasm32") {
    // }

    // TODO Run cmake or emcmake depends on target
    // mkdir build
    // cd build
    // cmake ../
    // cmake --build . -- -j 6
    // Create build dir
    // fs::create_dir_all("../build").into_diagnostic()?;


    // TODO Revisit
    // Rebuild of sources are changed
    println!("cargo:rerun-if-changed=../src");

    // println!("cargo:rustc-flags=-I/opt/homebrew/opt/llvm/include");
    // println!("cargo:rustc-flags=-I/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/");
    //
    // println!("cargo:rustc-link-search=/opt/homebrew/opt/llvm/lib");
    // println!("cargo:rustc-link-search=/opt/homebrew/opt/llvm/include");
    // println!("cargo:rustc-link-search=/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/");
    // println!("cargo:rustc-env=AR=/opt/homebrew/opt/llvm/bin/llvm-ar");
    // println!("cargo:rustc-env=AR=/opt/homebrew/bin/emar");
    //
    // // Link BLS signatures library and dependencies
    //
    println!("cargo:rustc-link-search={}", abs("../js_build/_deps/sodium-build"));
    println!("cargo:rustc-link-lib=static=sodium");

    println!("cargo:rustc-link-search={}", abs("../js_build/_deps/relic-build/lib"));
    println!("cargo:rustc-link-lib=static=relic_s");

    println!("cargo:rustc-link-search=/opt/homebrew/lib");
    println!("cargo:rustc-link-lib=static=gmp");

    println!("cargo:rustc-link-search={}", abs("../js_build/src"));
    println!("cargo:rustc-link-lib=static=bls-dash");

    // println!("cargo:rustc-link-args=");
    // println!("cargo:rustc-flags='-C relocation-model=pic -C link-arg=-shared -C link-arg=--imported-memory -C link-arg=--no-fatal-warnings'");
    //
    let include_paths = [
        std::path::PathBuf::from("../src"),
        std::path::PathBuf::from("../js_build/_deps/relic-src/include"),
        std::path::PathBuf::from("../js_build/_deps/relic-build/include"),
        std::path::PathBuf::from("../js_build/src"),
        // std::path::PathBuf::from("/usr/include"),
        // std::path::PathBuf::from("/opt/homebrew/opt/llvm/include"),

        // TODO Read from compiler_depend.make

        // std::path::PathBuf::from("/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/"),
        // std::path::PathBuf::from("/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/"),

        std::path::PathBuf::from("/opt/homebrew/Cellar/emscripten/3.1.19/libexec/cache/sysroot/include/c++/v1/"),
        std::path::PathBuf::from("/opt/homebrew/Cellar/emscripten/3.1.19/libexec/cache/sysroot/include/"),

        std::path::PathBuf::from("/opt/homebrew/include/"), // TODO gmp.h Use CMake generated files to get this path
    ];

    let mut build = autocxx_build::Builder::new("src/bindings.rs", &include_paths)
        .build()?;

    build.flag_if_supported("-std=c++14")
        // .archiver("/opt/homebrew/opt/llvm/bin/llvm-ar")
        .warnings(false) // Disable all Clang warnings
        .flag("-Wno-macro-redefined")// Should be disabled independently
        .compile("bls-signatures");

    Ok(())
}