use std::env;
use std::process::Command;

fn main() {
    // Respect PYO3_PYTHON if set (used by CI to target specific Python)
    println!("cargo:rerun-if-env-changed=PYO3_PYTHON");
    if let Ok(pyo3_python) = env::var("PYO3_PYTHON") {
        let output = Command::new(&pyo3_python)
            .args(["-c", "import sys; import sysconfig; print(sys.version_info.major, sys.version_info.minor, sysconfig.get_config_var('LIBDIR'), sysconfig.get_config_var('LDLIBRARY'), sysconfig.get_config_var('INCLUDEPY'))"])
            .output();

        if let Ok(output) = output {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = output_str.split_whitespace().collect();
            if parts.len() >= 5 {
                println!("cargo:include={}", parts[4]);
                println!("cargo:cflag=-I{}", parts[4]);
                println!("cargo:rustc-link-search=native={}", parts[2]);
                println!(
                    "cargo:rustc-env=PYO3_CROSS_PYTHON_VERSION={}.{}",
                    parts[0], parts[1]
                );
                println!("cargo:rustc-env=PYO3_CROSS_LIB_DIR={}", parts[2]);
                return;
            }
        }
    }

    // Default: use python3 from PATH
    let python = env::var("PYTHON").unwrap_or_else(|_| "python3".to_string());
    let output = Command::new(&python)
        .args(["-c", "import sys; import sysconfig; print(sys.version_info.major, sys.version_info.minor, sysconfig.get_config_var('LIBDIR'), sysconfig.get_config_var('LDLIBRARY'), sysconfig.get_config_var('INCLUDEPY'))"])
        .output()
        .expect("Failed to execute python. Make sure Python development headers are installed.");

    let output_str = String::from_utf8_lossy(&output.stdout);

    if output.status.success() {
        let parts: Vec<&str> = output_str.split_whitespace().collect();
        if parts.len() >= 5 {
            let major = parts[0];
            let minor = parts[1];
            let libdir = parts[2];
            let ldlibrary = parts[3];
            let includepy = parts[4];
            let libname = ldlibrary.trim_start_matches("lib").trim_end_matches(".so");

            let abi3_enabled =
                env::var("DEP_PYO3_ABI3").is_ok() || env::var("DEP_PYO3_ABI3_PY312").is_ok();

            println!("cargo:include={}", includepy);
            println!("cargo:cflag=-I{}", includepy);

            if !abi3_enabled {
                println!("cargo:rustc-link-search=native={}", libdir);
                println!("cargo:rustc-link-lib={}", libname);
            } else {
                println!("cargo:rustc-link-search=native={}", libdir);
            }

            println!(
                "cargo:rustc-env=PYO3_CROSS_PYTHON_VERSION={}.{}",
                major, minor
            );
            println!("cargo:rustc-env=PYO3_CROSS_LIB_DIR={}", libdir);
            return;
        }
    }

    // Fallback: try common system paths
    let fallback_paths = [
        "/usr/lib/x86_64-linux-gnu",
        "/usr/lib/aarch64-linux-gnu",
        "/usr/lib",
    ];

    for path in &fallback_paths {
        if std::path::Path::new(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
        }
    }

    println!("cargo:rustc-link-lib=python3");
    println!("cargo:rustc-env=PYO3_CROSS_PYTHON_VERSION=3.13");
    println!("cargo:rustc-env=PYO3_CROSS_LIB_DIR=/usr/lib");
}
