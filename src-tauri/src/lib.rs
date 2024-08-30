use lipu::{Item, Lipu, Metadata};
use serde::Serialize;
use tauri::{Builder, Manager, State};
use tokio::sync::Mutex;

mod lipu;

#[derive(Serialize)]
enum BackendError {
    CoreError(lipu::Error),
}

impl From<lipu::Error> for BackendError {
    fn from(value: lipu::Error) -> Self {
        Self::CoreError(value)
    }
}

#[tauri::command]
async fn add_feed(url: String, state: State<'_, Mutex<Lipu>>) -> Result<(), BackendError> {
    let mut lipu = state.lock().await;

    lipu.add_feed(url);
    lipu.write_to_disk().await?;

    Ok(())
}

#[tauri::command]
async fn refresh(state: State<'_, Mutex<Lipu>>) -> Result<(), BackendError> {
    state
        .lock()
        .await
        .refresh()
        .await
        .map_err(BackendError::from)
}

#[tauri::command]
async fn list(state: State<'_, Mutex<Lipu>>) -> Result<Vec<Metadata>, BackendError> {
    Ok(state.lock().await.list())
}

#[tauri::command]
async fn search(
    query: String,
    state: State<'_, Mutex<Lipu>>,
) -> Result<Vec<Metadata>, BackendError> {
    Ok(state.lock().await.search(&query))
}

#[tauri::command]
async fn load(item_id: String, state: State<'_, Mutex<Lipu>>) -> Result<Item, BackendError> {
    state
        .lock()
        .await
        .load(&item_id)
        .ok_or(BackendError::CoreError(lipu::Error::NotFound))
}

#[tauri::command]
async fn download_item(item_id: String, state: State<'_, Mutex<Lipu>>) -> Result<(), BackendError> {
    let mut lipu = state.lock().await;

    lipu.download_item(&item_id).await?;
    lipu.write_to_disk().await?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Hello, world!");

    Builder::default()
        .setup(|app| {
            let mut data_dir = app
                .path()
                .app_data_dir()
                .expect("Access to the filesystem denied, exiting");
            data_dir.push("lipu");

            std::fs::create_dir_all(&data_dir).expect("Access to filesystem denied, exiting");

            println!(
                "[INFO] the directory I'll create files in is `{}`",
                data_dir.as_os_str().to_str().unwrap_or("non-existant")
            );

            app.manage(Mutex::new(Lipu::new(data_dir)));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            add_feed,
            refresh,
            list,
            search,
            load,
            download_item
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
