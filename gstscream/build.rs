
fn main() {
    gst_plugin_version_helper::info();

    println!("cargo:rustc-link-lib=dylib=scream");
}
