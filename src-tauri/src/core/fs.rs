use std::fs;
use std::path::{Path, PathBuf};

#[derive(serde::Serialize)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

pub fn list_directory(vault_root: &Path, sub_path: &str) -> Result<Vec<FileNode>, String> {
    let target_dir: PathBuf = if sub_path.is_empty() {
        vault_root.to_path_buf()
    } else {
        vault_root.join(sub_path)
    };

    let mut nodes = Vec::new();
    let entries = fs::read_dir(&target_dir).map_err(|e| e.to_string())?;

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') {
            continue;
        }

        let is_dir = path.is_dir();
        let rel_path = path
            .strip_prefix(vault_root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");

        nodes.push(FileNode {
            name,
            path: rel_path,
            is_dir,
        });
    }

    nodes.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name)));
    Ok(nodes)
}

pub fn read_note(vault_root: &Path, rel_path: &str) -> Result<String, String> {
    let file_path = vault_root.join(rel_path);
    fs::read_to_string(file_path).map_err(|e| e.to_string())
}

pub fn write_note(vault_root: &Path, rel_path: &str, content: &str) -> Result<(), String> {
    let file_path = vault_root.join(rel_path);

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::write(file_path, content).map_err(|e| e.to_string())
}

pub fn create_note(vault_root: &Path, rel_path: &str) -> Result<(), String> {
    let file_path = vault_root.join(rel_path);

    if file_path.exists() {
        return Err(format!("Note already exists: {}", rel_path));
    }

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::write(file_path, "").map_err(|e| e.to_string())
}

pub fn delete_note(vault_root: &Path, rel_path: &str) -> Result<(), String> {
    let file_path = vault_root.join(rel_path);

    if !file_path.exists() {
        return Err(format!("Note not found: {}", rel_path));
    }

    fs::remove_file(file_path).map_err(|e| e.to_string())
}
