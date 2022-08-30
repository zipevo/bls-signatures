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
    if Path::new(BUILD_PATH).exists() {
        fs::remove_dir_all(BUILD_PATH).expect("can't clean build directory");
    }

    fs::create_dir_all(BUILD_PATH).expect("can't create build directory");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

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

    // Build deps for bls-signatures
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

    let mut cc = cc::Build::new();

    let cpp_files: Vec<_> = glob::glob("c_binding/*.cpp")
        .expect("can't get list of cpp files")
        .filter_map(Result::ok)
        .collect();

    // let include_paths = [
    //     relic_src.as_str(),
    //     relic_build.as_str(),
    //     build_src.as_str(),
    //     src.as_str(),
    //
    //     "/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/c++/v1/",
    //     "/Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/",
    //
    //     // "/opt/homebrew/Cellar/emscripten/3.1.19/libexec/cache/sysroot/include/c++/v1/",
    //     // "/opt/homebrew/Cellar/emscripten/3.1.19/libexec/cache/sysroot/include/",
    //     "/opt/homebrew/opt/llvm/include",
    //     "/usr/include",
    //     "/opt/homebrew/include/",
    // ];

    cc.files(cpp_files)
        .includes(&include_paths)
        .flag_if_supported("-std=c++14");
    // .define("BLSALLOC_SODIUM", Some("1"))
    // .define("SODIUM_STATIC", Some("1"));

    if target_arch.eq("wasm32") {
        cc.flag_if_supported("-ffreestanding")
            .define("_LIBCPP_HAS_NO_THREADS", Some("1"));
    } else {
        cc.cpp(true);
    }

    if !cfg!(debug_assertions) {
        cc.opt_level(2);
    }

    cc.compile("bls-go-binding");

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

    // Create and write binding to src/bindings.rs
    if env::var("GENERATE_BINDING").is_ok() {
        let bindings = bindgen::Builder::default()
            .header("wrapper.h")
            // .detect_include_paths(true)
            // .dynamic_link_require_all(true)
            .clang_args(include_paths.iter().map(|path| String::from("-I") + path))
            .size_t_is_usize(true)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .raw_line("#![allow(non_upper_case_globals)]\n#![allow(non_camel_case_types)]\n#![allow(non_snake_case)]")
            .generate()
            .expect("Unable to generate bindings");

        let current_dir = env::current_dir().expect("should get current dir");

        let out_path = current_dir.join(PathBuf::from("src/bindings.rs"));

        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
    }
}
