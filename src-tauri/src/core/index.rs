use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;
use regex::Regex;

/// Scans all .md files in the vault and builds a backlinks index.
/// backlinks[target] = Vec<source> — which notes link TO `target`.
pub fn build_index(vault_path: &Path) -> HashMap<String, Vec<String>> {
    let re = Regex::new(r"\[\[([^\[\]]+)\]\]").unwrap();
    let mut backlinks: HashMap<String, Vec<String>> = HashMap::new();

    for entry in WalkDir::new(vault_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_file()
                && e.path().extension().is_some_and(|ext| ext == "md")
                && !e
                    .path()
                    .components()
                    .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
        })
    {
        let source = entry
            .path()
            .strip_prefix(vault_path)
            .unwrap_or(entry.path())
            .to_string_lossy()
            .to_string();

        let content = match std::fs::read_to_string(entry.path()) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for cap in re.captures_iter(&content) {
            let target = cap[1].trim().to_string();
            backlinks
                .entry(target)
                .or_default()
                .push(source.clone());
        }
    }

    backlinks
}
