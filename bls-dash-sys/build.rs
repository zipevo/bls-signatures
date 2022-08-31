use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs, io};

fn abs(path: &str) -> String {
    let path_buf = PathBuf::from(path);

    let path_abs = path_buf
        .canonicalize()
        .expect("should provide valid abs path");

    path_abs
        .to_str()
        .expect("should convert path to string")
        .to_owned()
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
    // Run cmake

    println!("Run cmake:");

    if Path::new(BUILD_PATH).exists() {
        fs::remove_dir_all(BUILD_PATH).expect("can't clean build directory");
    }

    fs::create_dir_all(BUILD_PATH).expect("can't create build directory");

    let cmake_output = create_cross_cmake_command()
        .current_dir(abs(BUILD_PATH))
        .arg("-DBUILD_BLS_PYTHON_BINDINGS=0")
        .arg("-DBUILD_BLS_TESTS=0")
        .arg("-DBUILD_BLS_BENCHMARKS=0")
        .arg("-DBUILD_BLS_JS_BINDINGS=0")
        .arg("..")
        .output()
        .expect("can't run cmake");

    io::stdout()
        .write_all(&cmake_output.stdout)
        .expect("should write output");

    io::stderr()
        .write_all(&cmake_output.stderr)
        .expect("should write output");

    assert!(cmake_output.status.success());

    // Build deps for bls-signatures

    println!("Build dependencies:");

    let build_output = Command::new("cmake")
        .args(["--build", ".", "--", "-j", "6"])
        .current_dir(BUILD_PATH)
        .output()
        .expect("can't build bls-signatures deps");

    io::stdout()
        .write_all(&build_output.stdout)
        .expect("should write output");

    io::stderr()
        .write_all(&build_output.stderr)
        .expect("should write output");

    assert!(build_output.status.success());

    // Collect include paths
    let include_paths_file_path = PathBuf::from(BUILD_PATH).join("include_paths.txt");

    let include_paths =
        fs::read_to_string(include_paths_file_path).expect("should read include paths from file");

    let mut include_paths: Vec<_> = include_paths
        .split(';')
        .into_iter()
        .filter(|path| !path.is_empty())
        .collect();

    let relic_src = abs("../build/_deps/relic-src/include");
    let relic_build = abs("../build/_deps/relic-build/include");
    let build_src = abs("../build/src");
    let src = abs("../src");

    include_paths.extend([
        relic_src.as_str(),
        relic_build.as_str(),
        build_src.as_str(),
        src.as_str(),
    ]);

    // Build c binding

    println!("Build C binding:");

    let mut cc = cc::Build::new();

    let cpp_files: Vec<_> = glob::glob("c_binding/*.cpp")
        .expect("can't get list of cpp files")
        .filter_map(Result::ok)
        .collect();

    cc.files(cpp_files)
        .includes(&include_paths)
        .cpp(true)
        .flag_if_supported("-std=c++14");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    // Fix homebrew LLVM installation issue
    if env::consts::OS == "macos" && target_arch == "wasm32" {
        cc.archiver("llvm-ar");
    }

    if target_arch.eq("wasm32") {
        cc.flag_if_supported("-ffreestanding")
            .define("_LIBCPP_HAS_NO_THREADS", Some("1"));
    }

    if !cfg!(debug_assertions) {
        cc.opt_level(2);
    }

    cc.compile("bls-dash-sys");

    // Link dependencies
    println!(
        "cargo:rustc-link-search={}",
        abs("../build/_deps/sodium-build")
    );

    println!("cargo:rustc-link-lib=static=sodium");

    println!(
        "cargo:rustc-link-search={}",
        abs("../build/_deps/relic-build/lib")
    );

    println!("cargo:rustc-link-lib=static=relic_s");

    // Link GMP if exists
    let gmp_libraries_file_path = PathBuf::from(BUILD_PATH).join("gmp_libraries.txt");

    if gmp_libraries_file_path.exists() {
        let gmp_libraries_string = fs::read_to_string(gmp_libraries_file_path)
            .expect("should read gmp includes from file");

        let gmp_libraries_path = PathBuf::from(gmp_libraries_string);

        let gmp_libraries_parent_path = gmp_libraries_path
            .parent()
            .expect("can't get gmp libraries parent dir");

        println!(
            "cargo:rustc-link-search={}",
            gmp_libraries_parent_path.display()
        );

        println!("cargo:rustc-link-lib=static=gmp");
    }

    println!("cargo:rustc-link-search={}", abs("../build/src"));
    println!("cargo:rustc-link-lib=static=bls-dash");

    // Generate rust code for c binding to src/lib.rs
    if env::var("GENERATE_C_BINDING").is_ok() {
        println!("Generate C binding for rust:");

        let bindings = bindgen::Builder::default()
            .header("c_binding/blschia.h")
            // .header("c_binding/error.h")
            .header("c_binding/elements.h")
            .header("c_binding/privatekey.h")
            .header("c_binding/schemes.h")
            .header("c_binding/threshold.h")
            // .header("c_binding/utils.hpp")
            // .clang_arg("-xc++")
            .size_t_is_usize(true)
            // /Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1;
            // /Library/Developer/CommandLineTools/usr/lib/clang/13.1.6/include;
            // /Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include;
            // /Library/Developer/CommandLineTools/usr/include;
            // /opt/homebrew/include;
            // .clang_args([
            // "-I/Library/Developer/CommandLineTools/usr/include/c++/v1",
            // "-I/Library/Developer/CommandLineTools/usr/lib/clang/13.1.6/include",
            // "-I/opt/homebrew/Cellar/llvm/14.0.6_1/include", // "-I/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include",
            // "-I/Library/Developer/CommandLineTools/usr/include",
            // "-I/opt/homebrew/include",
            // ])
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .disable_header_comment()
            .raw_line("#![allow(non_upper_case_globals)]")
            .raw_line("#![allow(non_camel_case_types)]")
            .raw_line("#![allow(non_snake_case)]")
            .generate()
            .expect("Unable to generate bindings");

        let current_dir = env::current_dir().expect("should get current dir");

        let out_path = current_dir.join(PathBuf::from("src/lib.rs"));

        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
    }
}
