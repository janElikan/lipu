use lipu::{Lipu, Metadata};
use serde::Serialize;
use tauri::{Builder, Manager, State};
use tokio::sync::Mutex;

mod lipu;

#[derive(Serialize)]
enum BackendError {
    CoreError(lipu::Error),
}

#[tauri::command]
async fn add_feed(url: String, state: State<'_, Mutex<Lipu>>) -> Result<(), BackendError> {
    state.lock().await.add_feed(url);

    Ok(())
}

#[tauri::command]
async fn refresh(state: State<'_, Mutex<Lipu>>) -> Result<(), BackendError> {
    state.lock().await.refresh().await.map_err(|why| BackendError::CoreError(why))
}

#[tauri::command]
async fn list(state: State<'_, Mutex<Lipu>>) -> Result<Vec<Metadata>, BackendError> {
    Ok(state.lock().await.list())
}

#[tauri::command]
async fn search(query: String, state: State<'_, Mutex<Lipu>>) -> Result<Vec<Metadata>, BackendError> {
    Ok(state.lock().await.search(&query))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(Lipu::new()));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![add_feed, refresh, list, search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
