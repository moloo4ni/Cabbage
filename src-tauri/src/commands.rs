use tauri::State;
use std::fs;
use std::path::Path;
use crate::core::index::AppState;
use crate::git::cli;

#[tauri::command]
pub fn read_note(state: State<AppState>, path: String) -> Result<String, String> {
    // Безопасность: убедиться, что путь внутри vault (path traversal protection)
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_note(state: State<AppState>, path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| e.to_string())?;
    
    let vault = state.vault_path.lock().unwrap().clone().unwrap();
    // Auto-commit into Git
    cli::auto_commit(Path::new(&vault), &path)?;
    
    Ok(())
}

#[tauri::command]
pub fn sync(state: State<AppState>) -> Result<String, String> {
    let vault = state.vault_path.lock().unwrap().clone().unwrap();
    let result = cli::sync_vault(Path::new(&vault))?;
    Ok(result.output)
}