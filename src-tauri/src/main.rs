// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod core;
mod commands;

use state::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState::new()) // Инициализируем наш State
        .invoke_handler(tauri::generate_handler![
            commands::open_vault,
            commands::list_directory,
            commands::read_note,
            commands::write_note,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}