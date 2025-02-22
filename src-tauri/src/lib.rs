use std::sync::Mutex;

use ollama_rs::Ollama;
use tauri::{App, Manager};
use tauri_plugin_shell::ShellExt;

// mod moondream_vision_example;
// mod qwen;

#[allow(unused)]
#[derive(Default)]
struct AppData {
    ollama: Ollama,
    loaded_model: &'static str,
    available_models: [&'static str; 2],
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app|{
            let available_models = ["hf.co/openbmb/MiniCPM-V-2_6-gguf:IQ3_S", "deepscaler"];
            app.manage(Mutex::new(AppData {
                ollama: Ollama::default(),
                loaded_model: available_models[0],
                available_models,
            }));
            spawn_ollama(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn spawn_ollama(app: &mut App) {
    let sidecar_command = app.shell().sidecar("ollama").unwrap().arg("serve");
    let (mut _rx, mut _child) = sidecar_command.spawn().expect("Failed to spawn sidecar");

    // tauri::async_runtime::spawn(async move {
    //     // read events such as stdout
    //     while let Some(event) = rx.recv().await {
    //         if let CommandEvent::Stdout(line_bytes) = event {
    //             let line = String::from_utf8_lossy(&line_bytes);
    //             window
    //                 .emit("message", Some(format!("'{}'", line)))
    //                 .expect("failed to emit event");
    //             // write to stdin
    //             child.write("message from Rust\n".as_bytes()).unwrap();
    //         }
    //     }
    // });
}
