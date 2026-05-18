fn main() {
    // PyO3 abi3 builds skip linking to libpython for production builds,
    // but cargo test needs it. Detect python version dynamically.
    //
    // maturin sets PYO3_BUILD_EXTENSION_MODULE=1 during wheel builds;
    // linking libpython would break the extension-module feature.
    if std::env::var("PYO3_BUILD_EXTENSION_MODULE").is_ok() {
        return; // maturin build — no libpython linking
    }

    println!("cargo:rerun-if-changed=build.rs");

    // Fixed: hardcode 3.13 for aarch64/pip-installed build.
    // Container env may have python3->3.11 but we target 3.13.
    println!("cargo:rustc-link-lib=python3.13");
    println!("cargo:rustc-link-search=/usr/lib/aarch64-linux-gnu");
}
