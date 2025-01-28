use std::sync::Mutex;
use tauri::{Manager, State};
use tokio::sync::Mutex as AsyncMutex;

mod state;
use state::AppState;

mod ssh_client;
use ssh_client::*;

mod dfu_writer;
use dfu_writer::get_dfu_devices;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str, state: State<'_, Mutex<AppState>>) -> String {
    let mut state = state.lock().unwrap();
    state.test = name.to_string();
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_name(state: State<'_, Mutex<AppState>>) -> String {
    let state = state.lock().unwrap();
    format!("Get Name {}!", state.test)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            app.manage(AsyncMutex::new(SessionState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_name,
            connect_ssh,
            disconnect_ssh,
            send_ssh,
            get_dfu_devices,
            klipper_can_interface,
            klipper_can_uuid_list,
            klipper_send_fw,
            klipper_send_fluidd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
