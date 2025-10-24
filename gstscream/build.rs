extern crate cmake;
use cmake::Config;

fn main() {
    gst_plugin_version_helper::info();
    let crate_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let lib_dir = crate_dir.join("../lib").canonicalize().unwrap();

    Config::new("..")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build_target("scream")
        .profile("Release")
        .build();


    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=scream");
    println!("cargo:rustc-link-lib=c++");
}
