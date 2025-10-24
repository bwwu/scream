extern crate cmake;
use std::process::Output;

use cmake::Config;

fn main() {
    gst_plugin_version_helper::info();
    let crate_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:warning=FUCK {}", crate_dir.display());
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let lib_dir = out_dir.join("lib");
    // out_dir.join("../lib").canonicalize().unwrap();
    

    let cmakedir = Config::new("..")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .define("BUILD_SHARED_LIBS", "OFF")
        // .define("CMAKE_INSTALL_PREFIX", &out_dir)
        // .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", &out_dir.canonicalize().unwrap())
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY", lib_dir.to_str().unwrap())
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", lib_dir.to_str().unwrap())
        .build_target("scream")
        .profile("Release")
        .build();
    println!("cargo:warning=cmakedir={}", cmakedir.display());


    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=scream");
    println!("cargo:rustc-link-lib=c++");
}
