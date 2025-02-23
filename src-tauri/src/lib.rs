// use std::sync::Mutex;
mod ollama_helpers;

use ollama_rs::Ollama;
use tauri::Manager;
use tokio::sync::Mutex;

// mod moondream_vision_example;
// mod qwen;

#[allow(unused)]
#[derive(Default)]
pub struct AppData {
    ollama: Ollama,
    loaded_model: &'static str,
    available_models: [&'static str; 2],
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, ollama_helpers::list_available_models])
        .setup(|app| {
            let available_models = ["hf.co/openbmb/MiniCPM-V-2_6-gguf:IQ3_S", "deepscaler"];
            app.manage(Mutex::new(AppData {
                ollama: Ollama::default(),
                loaded_model: available_models[0],
                available_models,
            }));
            ollama_helpers::spawn_ollama(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// --------------------------------------------------------------------------
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
