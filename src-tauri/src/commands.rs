use std::path::PathBuf;
use tauri::{State, Manager};
use crate::state::AppState;
use crate::core::{fs, index};
use crate::git::cli;

// ── Vault ─────────────────────────────────────────────────────────────────────

/// Opens a folder-picker dialog and sets the active vault.
/// Ensures the selected directory is a git repository (init if needed).
#[tauri::command]
pub async fn pick_and_open_vault(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    use tauri::api::dialog::blocking::FileDialogBuilder;

    let path = FileDialogBuilder::new()
        .set_title("Open Vault")
        .pick_folder()
        .ok_or("No folder selected")?;

    open_vault_path(app, state, path.to_string_lossy().to_string()).await
}

/// Opens a vault at a specific path (useful for reopening recent vaults).
#[tauri::command]
pub async fn open_vault(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    path: String,
) -> Result<String, String> {
    open_vault_path(app, state, path).await
}

async fn open_vault_path(
    _app: tauri::AppHandle,
    state: State<'_, AppState>,
    path: String,
) -> Result<String, String> {
    let vault_path = PathBuf::from(&path);

    if !vault_path.exists() || !vault_path.is_dir() {
        return Err("Invalid vault path".into());
    }

    cli::ensure_git_repo(&vault_path)?;

    let backlinks = index::build_index(&vault_path);
    *state.current_vault.lock().unwrap() = Some(vault_path.clone());
    *state.backlinks.lock().unwrap() = backlinks;

    Ok(path)
}

// ── File tree ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn list_directory(
    state: State<'_, AppState>,
    sub_path: String,
) -> Result<Vec<fs::FileNode>, String> {
    let lock = state.current_vault.lock().unwrap();
    let root = lock.as_ref().ok_or("Vault not opened")?;
    fs::list_directory(root, &sub_path)
}

// ── Note CRUD ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn read_note(state: State<'_, AppState>, rel_path: String) -> Result<String, String> {
    let lock = state.current_vault.lock().unwrap();
    let root = lock.as_ref().ok_or("Vault not opened")?;
    fs::read_note(root, &rel_path)
}

#[tauri::command]
pub fn write_note(
    state: State<'_, AppState>,
    rel_path: String,
    content: String,
) -> Result<(), String> {
    let lock = state.current_vault.lock().unwrap();
    let root = lock.as_ref().ok_or("Vault not opened")?;
    let root = root.clone();
    drop(lock);

    fs::write_note(&root, &rel_path, &content)?;
    cli::auto_commit(&root, &rel_path)?;

    // Rebuild backlinks index after save
    let fresh = index::build_index(&root);
    *state.backlinks.lock().unwrap() = fresh;

    Ok(())
}

#[tauri::command]
pub fn create_note(
    state: State<'_, AppState>,
    rel_path: String,
) -> Result<(), String> {
    let lock = state.current_vault.lock().unwrap();
    let root = lock.as_ref().ok_or("Vault not opened")?;
    let root = root.clone();
    drop(lock);

    fs::create_note(&root, &rel_path)?;
    cli::auto_commit(&root, &rel_path)?;
    Ok(())
}

#[tauri::command]
pub fn delete_note(
    state: State<'_, AppState>,
    rel_path: String,
) -> Result<(), String> {
    let lock = state.current_vault.lock().unwrap();
    let root = lock.as_ref().ok_or("Vault not opened")?;
    let root = root.clone();
    drop(lock);

    fs::delete_note(&root, &rel_path)?;
    cli::auto_commit(&root, "--all")?;

    let fresh = index::build_index(&root);
    *state.backlinks.lock().unwrap() = fresh;

    Ok(())
}

// ── Backlinks ────────────────────────────────────────────────────────────────

/// Returns the list of notes that contain a [[link]] pointing to `note_name`.
#[tauri::command]
pub fn get_backlinks(
    state: State<'_, AppState>,
    note_name: String,
) -> Result<Vec<String>, String> {
    let lock = state.backlinks.lock().unwrap();
    Ok(lock.get(&note_name).cloned().unwrap_or_default())
}

// ── Git sync ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn sync(state: State<'_, AppState>) -> Result<cli::GitResult, String> {
    let lock = state.current_vault.lock().unwrap();
    let root = lock.as_ref().ok_or("Vault not opened")?;
    let root = root.clone();
    drop(lock);

    cli::sync_vault(&root)
}
