extern crate cmake;
use cmake::Config;

fn main() {
    let crate_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    Config::new(&crate_dir)
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .define("BUILD_SHARED_LIBS", "ON")
        .build_target("scream")
        .profile("Release")
        .build();

    let lib_dir = crate_dir.join("lib");
    if !lib_dir.exists() {
        panic!("Library output directory does not exist: {}", lib_dir.display());
    }
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=scream");
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:warning=libscream crate dir {}", crate_dir.display());
    println!("cargo:root={}", crate_dir.display());
}
