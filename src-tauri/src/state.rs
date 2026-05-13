use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Default)]
pub struct AppState {
    // Текущий открытый vault. Mutex нужен для потокобезопасности в Tauri.
    pub current_vault: Mutex<Option<PathBuf>>,
    // В будущем здесь будет кэш для [[ссылок]] (graph index)
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_vault: Mutex::new(None),
        }
    }
}