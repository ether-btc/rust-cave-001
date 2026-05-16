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

    let python_ver = std::process::Command::new("python3")
        .arg("-c")
        .arg("import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')")
        .output();

    match python_ver {
        Ok(output) if output.status.success() => {
            let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !ver.is_empty() {
                println!("cargo:rustc-link-lib=python{}", ver);
                println!("cargo:rustc-link-search=/usr/lib/aarch64-linux-gnu");
            }
        }
        _ => {
            eprintln!("cargo:warning=python3 not found — cargo test linking may fail");
        }
    }
}
