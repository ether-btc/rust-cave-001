fn main() {
    println!("cargo:warning=Build script started");
    
    let python = "python3";
    let output = std::process::Command::new(python)
        .args(&["-c", "import sysconfig; print(sysconfig.get_config_var(\"LIBPC\"))"])
        .output()
        .expect("Failed to execute python3");
    
    let libpc = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let status = output.status;
    
    println!("cargo: warning=Python command status: {:?}", status);
    println!("cargo: warning=Python output: {:?}", libpc);
    
    if !libpc.is_empty() {
        println!("cargo:rustc-link-search=native={}", libpc);
        println!("cargo:rustc-link-lib=dylib=python3.13");
    } else {
        println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
        println!("cargo:rustc-link-lib=dylib=python3.13");
    }
    
    println!("cargo: warning=Build script finished");
}
