use std::sync::OnceLock;

#[derive(Debug)]
pub struct Config {
    pub server_address: String,
}

// Global static instance of Config
static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    // Initialize CONFIG on first access
    CONFIG.get_or_init(|| Config {
        server_address: env!("SERVER_ADDRRES").to_string(),
    })
}
