use std::path::Path;
use std::process::Command;

#[derive(Debug, serde::Serialize)]
pub struct GitResult {
    pub success: bool,
    pub output: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub timestamp: String,
    pub author: String,
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

/// Returns the commit history for a specific file (up to 50 entries).
/// Uses ASCII Unit Separator (0x1F) as field delimiter to avoid
/// collisions with commit message content.
pub fn get_note_history(vault_path: &Path, rel_path: &str) -> Result<Vec<CommitInfo>, String> {
    let format = "%H\x1f%s\x1f%aI\x1f%an";
    let res = git_exec(
        vault_path,
        &["log", "--follow", &format!("--format={}", format), "-n", "50", "--", rel_path],
    )?;

    if !res.success {
        return Err(format!("git log failed: {}", res.output));
    }

    let commits = res.output
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut parts = line.splitn(4, '\x1f');
            CommitInfo {
                hash:      parts.next().unwrap_or("").to_string(),
                message:   parts.next().unwrap_or("").to_string(),
                timestamp: parts.next().unwrap_or("").to_string(),
                author:    parts.next().unwrap_or("").to_string(),
            }
        })
        .collect();

    Ok(commits)
}

/// Returns the raw content of a file at a specific commit.
pub fn get_note_at_commit(
    vault_path: &Path,
    commit_hash: &str,
    rel_path: &str,
) -> Result<String, String> {
    let object = format!("{}:{}", commit_hash, rel_path);
    let res = git_exec(vault_path, &["show", &object])?;

    if !res.success {
        return Err(format!("git show failed: {}", res.output));
    }

    Ok(res.output)
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
