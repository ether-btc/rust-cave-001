fn main() {
    let python = "python3";

    let output = std::process::Command::new(python)
        .args(&["-c", "import sys; import sysconfig; print(sys.version_info.major, sys.version_info.minor, sysconfig.get_config_var(\"LIBDIR\"), sysconfig.get_config_var(\"LDLIBRARY\"), sysconfig.get_config_var(\"INCLUDEPY\"))"])
        .output()
        .expect("Failed to execute python");

    let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if output.status.success() {
        let parts: Vec<&str> = output_str.split_whitespace().collect();
        if parts.len() >= 5 {
            let libdir = parts[2];
            let ldlibrary = parts[3];
            let includepy = parts[4];
            let libname = ldlibrary.trim_start_matches("lib").trim_end_matches(".so");

            let abi3_enabled = std::env::var("DEP_PYO3_ABI3").is_ok()
                || std::env::var("DEP_PYO3_ABI3_PY312").is_ok();

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
                parts[0], parts[1]
            );
            println!("cargo:rustc-env=PYO3_CROSS_LIB_DIR={}", libdir);
            return;
        }
    }

    // Fallback for Python 3.13 on RPi
    println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
    println!("cargo:rustc-link-lib=python3.13");
    println!("cargo:include=/usr/include/python3.13");
    println!("cargo:cflag=-I/usr/include/python3.13");
    println!("cargo:rustc-env=PYO3_CROSS_PYTHON_VERSION=3.13");
    println!("cargo:rustc-env=PYO3_CROSS_LIB_DIR=/usr/lib/aarch64-linux-gnu");
}
