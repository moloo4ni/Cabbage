use std::fs;
use std::path::{Path, PathBuf};

#[derive(serde::Serialize)]
pub struct FileNode {
    pub name: String,
    pub path: String, // Относительный путь от корня vault
    pub is_dir: bool,
}

pub fn list_directory(vault_root: &Path, sub_path: &str) -> Result<Vec<FileNode>, String> {
    let target_dir = vault_root.join(sub_path);
    
    let mut nodes = Vec::new();
    let entries = fs::read_dir(target_dir).map_err(|e| e.to_string())?;

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        
        // Игнорируем скрытые файлы и папку .git
        if name.starts_with('.') {
            continue;
        }

        let is_dir = path.is_dir();
        // Строим относительный путь для безопасной передачи на фронтенд
        let rel_path = path.strip_prefix(vault_root)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

        nodes.push(FileNode { name, path: rel_path, is_dir });
    }

    // Сортировка: папки сверху, файлы снизу
    nodes.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name)));
    Ok(nodes)
}

pub fn read_note(vault_root: &Path, rel_path: &str) -> Result<String, String> {
    let file_path = vault_root.join(rel_path);
    fs::read_to_string(file_path).map_err(|e| e.to_string())
}

pub fn write_note(vault_root: &Path, rel_path: &str, content: &str) -> Result<(), String> {
    let file_path = vault_root.join(rel_path);
    
    // Создаем папки, если их нет
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::write(file_path, content).map_err(|e| e.to_string())
}