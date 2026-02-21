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

    tauri_build::build()
}
