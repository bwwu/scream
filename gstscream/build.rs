extern crate cmake;

use cmake::Config;

fn main() {
    gst_plugin_version_helper::info();
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let lib_dir = out_dir.join("lib");

    let cmakedir = Config::new("..")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY", lib_dir.to_str().unwrap())
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", lib_dir.to_str().unwrap())
        .build_target("scream")
        .profile("Release")
        .build();
    println!("cargo:warning=cmakedir={}", cmakedir.display());
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=scream");

    let target = std::env::var("TARGET").unwrap();
    if (target.contains("apple-darwin")) {
        println!("cargo:rustc-link-lib=c++");
    }

    let target: String = std::env::var("TARGET").unwrap();
    if !target.contains("apple") {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
