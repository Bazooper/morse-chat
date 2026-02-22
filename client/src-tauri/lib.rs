mod config;

#[tauri::command]
async fn greet(name: String) -> String {
    let mut url = config::config().server_address.clone();

    if name != "" {
        url.push_str(&format!("/{}", &name));
    }

    let text = match reqwest::get(&url).await {
        Ok(resp) => match resp.text().await {
            Ok(body) => body,
            Err(e) => format!("Failed to read response body: {}", e),
        },
        Err(e) => format!("Request failed: {}", e),
    };

    text
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
