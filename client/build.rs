fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let env_path = std::path::Path::new(&manifest_dir).join(".env");

    // Rebuild if .env changes
    println!("cargo:rerun-if-changed=.env");

    if let Ok(contents) = std::fs::read_to_string(&env_path) {
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once('=') {
                // Set environment variable for the build process
                println!("cargo:rustc-env={}={}", key.trim(), value.trim());
            }
        }
    }

    // Install frontend dependencies
    println!("cargo:rerun-if-changed=package.json");
    std::process::Command::new("npm")
        .args(&["install"])
        .status()
        .expect("Failed to install frontend dependencies");

    // Build the frontend
    println!("cargo:rerun-if-changed=src");
    std::process::Command::new("npm")
        .args(&["run", "build"])
        .status()
        .expect("Failed to build frontend");

    tauri_build::build()
}
