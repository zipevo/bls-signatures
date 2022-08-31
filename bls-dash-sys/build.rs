use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::{env, fs, io};

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

fn handle_command_output(output: Output) {
    io::stdout()
        .write_all(&output.stdout)
        .expect("should write output");

    io::stderr()
        .write_all(&output.stderr)
        .expect("should write output");

    assert!(output.status.success());
}

const BUILD_PATH: &str = "../build";

fn main() {
    let bls_dash_build_path = Path::new("../build")
        .canonicalize()
        .expect("can't get abs path");

    let bls_dash_src_path = Path::new("../src")
        .canonicalize()
        .expect("can't get abs path");

    let c_binding_path = Path::new("c_binding")
        .canonicalize()
        .expect("can't get abs path");

    // Run cmake

    println!("Run cmake:");

    if bls_dash_build_path.exists() {
        fs::remove_dir_all(&bls_dash_build_path).expect("can't clean build directory");
    }

    fs::create_dir_all(&bls_dash_build_path).expect("can't create build directory");

    let cmake_output = create_cross_cmake_command()
        .current_dir(&bls_dash_build_path)
        .arg("-DBUILD_BLS_PYTHON_BINDINGS=0")
        .arg("-DBUILD_BLS_TESTS=0")
        .arg("-DBUILD_BLS_BENCHMARKS=0")
        .arg("-DBUILD_BLS_JS_BINDINGS=0")
        .arg("..")
        .output()
        .expect("can't run cmake");

    handle_command_output(cmake_output);

    // Build deps for bls-signatures

    println!("Build dependencies:");

    let build_output = Command::new("cmake")
        .args(["--build", ".", "--", "-j", "6"])
        .current_dir(BUILD_PATH)
        .output()
        .expect("can't build bls-signatures deps");

    handle_command_output(build_output);

    // Collect include paths
    let include_paths_file_path = bls_dash_build_path.join("include_paths.txt");

    let include_paths =
        fs::read_to_string(include_paths_file_path).expect("should read include paths from file");

    let mut include_paths: Vec<_> = include_paths
        .split(';')
        .filter(|path| !path.is_empty())
        .map(|path| PathBuf::from(path))
        .collect();

    include_paths.extend([
        bls_dash_build_path.join("_deps/relic-src/include"),
        bls_dash_build_path.join("_deps/relic-build/include"),
        bls_dash_build_path.join("src"),
        bls_dash_src_path.clone(),
    ]);

    // Build c binding

    println!("Build C binding:");

    let mut cc = cc::Build::new();

    let cpp_files_mask = c_binding_path.join("*.cpp");

    let cpp_files: Vec<_> = glob::glob(cpp_files_mask.to_str().unwrap())
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
        bls_dash_build_path.join("_deps/sodium-build").display()
    );

    println!("cargo:rustc-link-lib=static=sodium");

    println!(
        "cargo:rustc-link-search={}",
        bls_dash_build_path.join("_deps/relic-build/lib").display()
    );

    println!("cargo:rustc-link-lib=static=relic_s");

    println!(
        "cargo:rustc-link-search={}",
        bls_dash_build_path.join("src").display()
    );

    println!("cargo:rustc-link-lib=static=bls-dash");

    // Link GMP if exists
    let gmp_libraries_file_path = bls_dash_build_path.join("gmp_libraries.txt");

    if gmp_libraries_file_path.exists() {
        let gmp_libraries_path = PathBuf::from(
            fs::read_to_string(gmp_libraries_file_path)
                .expect("should read gmp includes from file"),
        );

        let gmp_libraries_parent_path = gmp_libraries_path
            .parent()
            .expect("can't get gmp libraries parent dir");

        println!(
            "cargo:rustc-link-search={}",
            gmp_libraries_parent_path.display()
        );

        println!("cargo:rustc-link-lib=static=gmp");
    }

    // Generate rust code for c binding to src/lib.rs
    if env::var("GENERATE_C_BINDING").is_ok() {
        println!("Generate C binding for rust:");

        let bindings = bindgen::Builder::default()
            .header(c_binding_path.join("blschia.h").to_str().unwrap())
            // .header(c_binding_path.join("error.h"))
            .header(c_binding_path.join("elements.h").to_str().unwrap())
            .header(c_binding_path.join("privatekey.h").to_str().unwrap())
            .header(c_binding_path.join("schemes.h").to_str().unwrap())
            .header(c_binding_path.join("threshold.h").to_str().unwrap())
            // .header(c_binding_path.join("utils.h"))
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

    // Rerun build if files changed
    println!("cargo:rerun-if-changed={}", c_binding_path.display());
    println!("cargo:rerun-if-changed={}", bls_dash_src_path.display());
}
