mod git;
mod core;
mod commands;

use core::index::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::read_note,
            commands::write_note,
            commands::sync
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}