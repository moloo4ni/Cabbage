use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct AppState {
    pub current_vault: Mutex<Option<PathBuf>>,
    pub backlinks: Mutex<HashMap<String, Vec<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_vault: Mutex::new(None),
            backlinks: Mutex::new(HashMap::new()),
        }
    }
}
