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

    // Detect Python link flags dynamically so we work on any platform/arch.
    // python3-config --ldflags gives us -L flags for the Python library paths.
    let python_config = std::process::Command::new("python3-config")
        .arg("--ldflags")
        .output();

    let ver = std::process::Command::new("python3")
        .arg("-c")
        .arg("import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    if !ver.is_empty() {
        println!("cargo:rustc-link-lib=python{ver}");
    }

    if let Ok(output) = python_config {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse -L flags from python3-config --ldflags
            for part in output_str.split_whitespace() {
                if let Some(path) = part.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={path}");
                }
            }
        }
    }
}
