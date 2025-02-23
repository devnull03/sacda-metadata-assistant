use tauri::App;
use tauri_plugin_shell::ShellExt;
use tokio::sync::Mutex;

use crate::AppData;

pub fn spawn_ollama(app: &mut App) {
    let sidecar_command = app.shell().sidecar("ollama").unwrap().arg("serve");
    let (mut _rx, mut _child) = sidecar_command.spawn().expect("Failed to spawn sidecar");
}

#[tauri::command]
pub async fn list_available_models(state: tauri::State<'_, Mutex<AppData>>) -> Result<Vec<String>, ()> {
    let state = state.lock().await;

    let res = state.ollama.list_local_models().await.unwrap();

    Ok(res.iter().map(|e| e.name.to_string()).collect())
}
