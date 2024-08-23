use lipu::{Item, Lipu, Metadata};
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

#[tauri::command]
async fn load(item_id: String, state: State<'_, Mutex<Lipu>>) -> Result<Item, BackendError> {
    state.lock().await.load(&item_id).ok_or(BackendError::CoreError(lipu::Error::NotFound))
}

#[tauri::command]
async fn download_item(item_id: String, state: State<'_, Mutex<Lipu>>) -> Result<(), BackendError> {
    state.lock().await.download_item(&item_id).await.map_err(|why| BackendError::CoreError(why))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(Lipu::new()));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![add_feed, refresh, list, search, load, download_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
