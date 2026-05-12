use std::process::Command;
use std::path::Path;

#[derive(Debug, serde::Serialize)]
pub struct GitResult {
    pub success: bool,
    pub output: String,
}

pub fn git_exec(vault_path: &Path, args: &[&str]) -> Result<GitResult, String> {
    let output = Command::new("git")
        .current_dir(vault_path)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute git: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok(GitResult {
        success: output.status.success(),
        output: if output.status.success() { stdout } else { stderr },
    })
}

// "Человеческие" абстракции
pub fn auto_commit(vault_path: &Path, file_path: &str) -> Result<(), String> {
    git_exec(vault_path, &["add", file_path])?;
    // Делаем тихий коммит без вывода в UI
    git_exec(vault_path, &["commit", "-m", &format!("Update {}", file_path)])?;
    Ok(())
}

pub fn sync_vault(vault_path: &Path) -> Result<GitResult, String> {
    // 1. Fetch
    git_exec(vault_path, &["fetch", "origin"])?;
    // 2. Rebase (чистая история, без merge commits)
    let pull_res = git_exec(vault_path, &["pull", "--rebase", "origin", "main"]);
    if let Ok(res) = &pull_res {
        if !res.success {
            // TODO: Handle merge conflicts (git rebase --abort)
            git_exec(vault_path, &["rebase", "--abort"])?;
            return Err("Merge conflict detected. Manual resolution needed.".to_string());
        }
    }
    // 3. Push
    git_exec(vault_path, &["push", "origin", "main"])
}