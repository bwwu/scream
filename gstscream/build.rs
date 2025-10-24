use std::env;
use std::path::PathBuf;

fn main() {
    gst_plugin_version_helper::info();
    let lib_dir = env::var("DEP_LIBSCREAM_ROOT").unwrap();
    println!("cargo:warning=Library built at: {}", lib_dir);

    println!("cargo:rustc-link-lib=static=my_c_lib");
}
