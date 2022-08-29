use std::{env, fs, io};
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn abs(path: &str) -> String {
    let path_buf = PathBuf::from(path);

    let path_abs = path_buf.canonicalize().expect("should provide valid abs path");

    path_abs.to_str().expect("should convert path to string").to_owned()
}

fn create_cross_cmake_command() -> Command {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let mut command = if target_arch.eq("wasm32") {
        Command::new("emcmake")
    } else {
        Command::new("cmake")
    };

    if target_arch.eq("wasm32") {
        command.arg("cmake");
    }

    command
}

const BUILD_PATH: &str = "../build";

fn main() {
    // Prepare build
    fs::remove_dir_all(BUILD_PATH).expect("can't clean build directory");
    fs::create_dir_all(BUILD_PATH).expect("can't create build directory");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    create_cross_cmake_command()
        .current_dir(abs(BUILD_PATH))
        .arg("..")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()
        .expect("should prepare build files");

    // Build deps
    Command::new("cmake")
        .args(["--build", ".", "--", "-j", "6"])
        .current_dir(BUILD_PATH)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()
        .expect("should prepare build files");

    // Collect include paths
    // create_cross_cmake_command()


    // Command::new("")

    let mut cc = cc::Build::new();

    let library_path = Path::new("../go-bindings");

    let cpp_files = [
        abs("../go-bindings/blschia.cpp"),
        // "../go-bindings/elements.cpp",
        // "../go-bindings/privatekey.cpp",
        // "../go-bindings/schemes.cpp",
        // "../go-bindings/threshold.cpp",
        // "../go-bindings/utils.cpp",
    ];
    // [
    //     abs("../js_build/_deps/relic-src/include").as_str(),
    //     abs("../js_build/_deps/relic-build/include").as_str(),
    //     abs("../js_build/src").as_str(),
    //
    //     // "/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/",
    //     // "/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/",
    //
    //     "/opt/homebrew/Cellar/emscripten/3.1.19/libexec/cache/sysroot/include/c++/v1/",
    //     "/opt/homebrew/Cellar/emscripten/3.1.19/libexec/cache/sysroot/include/",
    //     "/opt/homebrew/opt/llvm/include",
    //     "/usr/include",
    //     "/opt/homebrew/include/",
    // ]
    //
    // cc.files(cpp_files)
    //     .includes(&include_paths)
    //     .flag_if_supported("-std=c++14");
    //     // .define("BLSALLOC_SODIUM", Some("1"))
    //     // .define("SODIUM_STATIC", Some("1"));
    //
    // if target_arch.eq("wasm32") {
    //     cc.flag_if_supported("-ffreestanding")
    //         .define("_LIBCPP_HAS_NO_THREADS", Some("1"));
    // } else {
    //     cc.cpp(true);
    // }
    //
    // if !cfg!(debug_assertions) {
    //     cc.opt_level(2);
    // }
    //
    // cc.compile("bls-go-binding");
    //
    // println!("cargo:rustc-link-search={}", abs("../js_build/_deps/sodium-build"));
    // println!("cargo:rustc-link-lib=static=sodium");
    //
    // println!("cargo:rustc-link-search={}", abs("../js_build/_deps/relic-build/lib"));
    // println!("cargo:rustc-link-lib=static=relic_s");
    //
    // println!("cargo:rustc-link-search=/opt/homebrew/lib");
    // println!("cargo:rustc-link-lib=static=gmp");
    //
    // println!("cargo:rustc-link-search={}", abs("../js_build/src"));
    // println!("cargo:rustc-link-lib=static=bls-dash");
    //
    // // Create and write binding to src/bindings.rs
    // if env::var("GENERATE_BINDING").is_ok() {
    //     let bindings = bindgen::Builder::default()
    //         // The input header we would like to generate
    //         // bindings for.
    //         .header("wrapper.h")
    //         // .detect_include_paths(true)
    //         // .dynamic_link_require_all(true)
    //         .clang_args(include_paths.iter().map(|path| String::from("-I") + path))
    //         .size_t_is_usize(true)
    //         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //         .generate()
    //         .expect("Unable to generate bindings");
    //
    //     let current_dir = env::current_dir().expect("should get current dir");
    //
    //         /*
    //     #![allow(non_upper_case_globals)]
    // #![allow(non_camel_case_types)]
    // #![allow(non_snake_case)]
    //      */
    //
    //     let out_path = current_dir.join(PathBuf::from("src/bindings.rs"));
    //
    //     bindings
    //         .write_to_file(out_path)
    //         .expect("Couldn't write bindings!");
    // }
}