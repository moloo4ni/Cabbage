use std::collections::HashMap;
use std::sync::Mutex;
use regex::Regex;

pub struct AppState {
    pub vault_path: Mutex<Option<String>>,
    // Target Note -> Vec<Source Notes> (для backlinks)
    pub backlinks: Mutex<HashMap<String, Vec<String>>>, 
}

impl AppState {
    pub fn new() -> Self {
        Self {
            vault_path: Mutex::new(None),
            backlinks: Mutex::new(HashMap::new()),
        }
    }

    pub fn build_index(&self, path: &str) {
        let mut links_map = HashMap::new();
        let re = Regex::new(r"\[\[(.*?)\]\]").unwrap();
        
        // Псевдокод: используем walkdir для обхода .md файлов
        // Читаем каждый файл
        // Находим все [[links]] через regex
        // links_map.entry(target).or_insert_with(Vec::new).push(current_file);
        
        *self.backlinks.lock().unwrap() = links_map;
    }
}