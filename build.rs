fn main() {
    // Print all environment variables for debugging
    for (key, value) in std::env::vars() {
        eprintln!("ENV: {}={}", key, value);
    }
    
    // Use the specific Python interpreter from the Hermes environment
    let python = "python3";
    
    // Get Python version and library information
    let output = std::process::Command::new(python)
        .args(&["-c", "import sys; import sysconfig; print(sys.version_info.major, sys.version_info.minor, sysconfig.get_config_var(\"LIBDIR\"), sysconfig.get_config_var(\"LDLIBRARY\"), sysconfig.get_config_var(\"INCLUDEPY\"), sysconfig.get_config_var(\"MACROS\"))"])
        .output()
        .expect("Failed to execute python");
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let status = output.status;
    
    // Debug output - use eprintln! so it goes to stderr and doesn't interfere with cargo directives
    eprintln!("Python command status: {:?}", status);
    eprintln!("Python output: {:?}", output_str);
    
    if status.success() && !output_str.is_empty() {
        let parts: Vec<&str> = output_str.trim().split_whitespace().collect();
        if parts.len() >= 6 {
            let major = parts[0];
            let minor = parts[1];
            let libdir = parts[2];
            let ldlibrary = parts[3];
            let includepy = parts[4];
            let macros = parts[5];
            
            // Extract library name without 'lib' prefix and without '.so' suffix
            let libname = ldlibrary.trim_start_matches("lib").trim_end_matches(".so");
            
            // Check if abi3 feature is enabled via environment variable
            let abi3_enabled = std::env::var("DEP_PYO3_ABI3").is_ok() || std::env::var("DEP_PYO3_ABI3_PY312").is_ok();
            eprintln!("DEBUG: abi3_enabled: {:?}", abi3_enabled);
            
            // Emit include path
            eprintln!("DEBUG: Emitting cargo:include={}", includepy);
            println!("cargo:include={}", includepy);
            eprintln!("DEBUG: Emitting cargo:cflag=-I{}", includepy);
            println!("cargo:cflag=-I{}", includepy);
            
            if !abi3_enabled {
                // Emit linking directives only if abi3 is NOT enabled
                eprintln!("DEBUG: Emitting cargo:rustc-link-search=native={}", libdir);
                println!("cargo:rustc-link-search=native={}", libdir);
                eprintln!("DEBUG: Emitting cargo:rustc-link-lib={}", libname);
                println!("cargo:rustc-link-lib={}", libname);
            } else {
                // For abi3, we may still need to specify the library search path
                eprintln!("DEBUG: Emitting cargo:rustc-link-search=native={}", libdir);
                println!("cargo:rustc-link-search=native={}", libdir);
            }
            
            // Set environment variables for PyO3 when cross-compiling
            eprintln!("DEBUG: Emitting cargo:rustc-env=PYO3_CROSS_PYTHON_VERSION={}.{}", major, minor);
            println!("cargo:rustc-env=PYO3_CROSS_PYTHON_VERSION={}.{}", major, minor);
            eprintln!("DEBUG: Emitting cargo:rustc-env=PYO3_CROSS_LIB_DIR={}", libdir);
            println!("cargo:rustc-env=PYO3_CROSS_LIB_DIR={}", libdir);
        } else {
            // Fallback to hardcoded paths
            eprintln!("DEBUG: Emitting cargo:include=/usr/include/python3.12");
            println!("cargo:include=/usr/include/python3.12");
            eprintln!("DEBUG: Emitting cargo:cflag=-I/usr/include/python3.12");
            println!("cargo:cflag=-I/usr/include/python3.12");
            
            // Check if abi3 is enabled
            let abi3_enabled = std::env::var("DEP_PYO3_ABI3").is_ok() || std::env::var("DEP_PYO3_ABI3_PY312").is_ok();
            eprintln!("DEBUG: abi3_enabled: {:?}", abi3_enabled);
            
            if !abi3_enabled {
                eprintln!("DEBUG: Emitting cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
                println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
                eprintln!("DEBUG: Emitting cargo:rustc-link-lib=python3.12");
                println!("cargo:rustc-link-lib=python3.12");
            } else {
                eprintln!("DEBUG: Emitting cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
                println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
            }
            
            eprintln!("DEBUG: Emitting cargo:rustc-env=PYO3_CROSS_PYTHON_VERSION=3.12");
            println!("cargo:rustc-env=PYO3_CROSS_PYTHON_VERSION=3.12");
            eprintln!("DEBUG: Emitting cargo:rustc-env=PYO3_CROSS_LIB_DIR=/usr/lib/aarch64-linux-gnu");
            println!("cargo:rustc-env=PYO3_CROSS_LIB_DIR=/usr/lib/aarch64-linux-gnu");
        }
    } else {
        // Fallback to hardcoded paths
        eprintln!("DEBUG: Emitting cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
        println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
        eprintln!("DEBUG: Emitting cargo:rustc-link-lib=python3.12");
        println!("cargo:rustc-link-lib=python3.12");
        eprintln!("DEBUG: Emitting cargo:include=/usr/include/python3.12");
        println!("cargo:include=/usr/include/python3.12");
        eprintln!("DEBUG: Emitting cargo:cflag=-I/usr/include/python3.12");
        println!("cargo:cflag=-I/usr/include/python3.12");
        
        // Check if abi3 is enabled
        let abi3_enabled = std::env::var("DEP_PYO3_ABI3").is_ok() || std::env::var("DEP_PYO3_ABI3_PY312").is_ok();
        eprintln!("DEBUG: abi3_enabled: {:?}", abi3_enabled);
        
        if !abi3_enabled {
            eprintln!("DEBUG: Emitting cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
            println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
            eprintln!("DEBUG: Emitting cargo:rustc-link-lib=python3.12");
            println!("cargo:rustc-link-lib=python3.12");
        } else {
            eprintln!("DEBUG: Emitting cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
            println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
        }
        
        eprintln!("DEBUG: Emitting cargo:rustc-env=PYO3_CROSS_PYTHON_VERSION=3.12");
        println!("cargo:rustc-env=PYO3_CROSS_PYTHON_VERSION=3.12");
        eprintln!("DEBUG: Emitting cargo:rustc-env=PYO3_CROSS_LIB_DIR=/usr/lib/aarch64-linux-gnu");
        println!("cargo:rustc-env=PYO3_CROSS_LIB_DIR=/usr/lib/aarch64-linux-gnu");
    }
    
    eprintln!("Build script finished");
}
