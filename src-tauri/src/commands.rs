use std::path::PathBuf;
use tauri::State;
use crate::state::AppState;
use crate::core::fs::{self, FileNode};

#[tauri::command]
pub fn open_vault(state: State<AppState>, path: String) -> Result<String, String> {
    let vault_path = PathBuf::from(&path);
    if !vault_path.exists() || !vault_path.is_dir() {
        return Err("Invalid vault path".into());
    }

    // TODO: Проверка на наличие .git, или выполнение `git init`

    *state.current_vault.lock().unwrap() = Some(vault_path);
    Ok(path)
}

#[tauri::command]
pub fn list_directory(state: State<AppState>, sub_path: String) -> Result<Vec<FileNode>, String> {
    let vault_lock = state.current_vault.lock().unwrap();
    let vault_root = vault_lock.as_ref().ok_or("Vault not opened")?;
    
    fs::list_directory(vault_root, &sub_path)
}

#[tauri::command]
pub fn read_note(state: State<AppState>, rel_path: String) -> Result<String, String> {
    let vault_lock = state.current_vault.lock().unwrap();
    let vault_root = vault_lock.as_ref().ok_or("Vault not opened")?;
    
    fs::read_note(vault_root, &rel_path)
}

#[tauri::command]
pub fn write_note(state: State<AppState>, rel_path: String, content: String) -> Result<(), String> {
    let vault_lock = state.current_vault.lock().unwrap();
    let vault_root = vault_lock.as_ref().ok_or("Vault not opened")?;
    
    fs::write_note(vault_root, &rel_path, &content)?;

    // TODO: Вызвать git add & git commit (Auto-save)

    Ok(())
}