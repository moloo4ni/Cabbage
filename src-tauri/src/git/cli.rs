use std::path::Path;
use std::process::Command;

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

/// Initialises a git repo at the given path if one does not already exist.
pub fn ensure_git_repo(vault_path: &Path) -> Result<(), String> {
    let git_dir = vault_path.join(".git");
    if !git_dir.exists() {
        let res = git_exec(vault_path, &["init"])?;
        if !res.success {
            return Err(format!("git init failed: {}", res.output));
        }
    }
    Ok(())
}

/// Stages and commits a single file with an automatic message.
pub fn auto_commit(vault_path: &Path, file_path: &str) -> Result<(), String> {
    let stage = git_exec(vault_path, &["add", file_path])?;
    if !stage.success {
        return Err(format!("git add failed: {}", stage.output));
    }

    // --allow-empty-message just in case; -q suppresses output
    let commit = git_exec(
        vault_path,
        &["commit", "-q", "--allow-empty", "-m", &format!("Update {}", file_path)],
    )?;

    // Exit code 1 with "nothing to commit" is not a real error
    if !commit.success && !commit.output.contains("nothing to commit") {
        return Err(format!("git commit failed: {}", commit.output));
    }

    Ok(())
}

/// fetch → pull --rebase → push. Aborts the rebase on conflict.
pub fn sync_vault(vault_path: &Path) -> Result<GitResult, String> {
    git_exec(vault_path, &["fetch", "origin"])?;

    let pull = git_exec(vault_path, &["pull", "--rebase", "origin", "main"])?;
    if !pull.success {
        git_exec(vault_path, &["rebase", "--abort"]).ok();
        return Err("Merge conflict detected. Sync aborted. Resolve manually.".to_string());
    }

    git_exec(vault_path, &["push", "origin", "main"])
}
