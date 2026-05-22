# Changelog

All notable changes to Cabbage are documented in this file.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

**Native Rust Git bindings**
- Replaced every `Command::new("git")` shell subprocess call with `git2` (libgit2 Rust bindings)
- `ensure_git_repo` → `Repository::init()` — no external `git` binary required for init
- `auto_commit` → index API (`add_path` / `remove_path` / `update_all` + `add_all`) +
  `repo.commit()`; silently skips when the tree matches HEAD (no "nothing to commit" subprocess)
- `get_note_history` → `revwalk` over the full history, diff each commit against its parent
  to find commits that touched the file; up to 50 results, newest-first
- `get_note_at_commit` → `repo.find_commit` → tree lookup → blob content; no shell round-trip
- `sync_vault` → native fetch with `RemoteCallbacks`, merge-analysis-driven integration
  (fast-forward or rebase), and native push; same error semantics as before
- Credential callback: SSH agent first, then falls back to `~/.ssh/id_ed25519` / `id_ecdsa` /
  `id_rsa`; platform-default credentials as final fallback
- Committer identity: reads from local git config; falls back to `Cabbage <cabbage@local>`
- `git2` added as a dependency with `vendored-libgit2` (no system libgit2 required) and
  `ssh` (SSH remote support via libssh2)

**Previously merged — Graph view**
- New `get_graph` Tauri command: walks the vault for all `.md` files and converts
  the backlinks index into a list of `GraphNode` / `GraphEdge` structs
- `GraphView.svelte`: canvas-based force-directed graph; no extra runtime
  dependencies — repulsion, spring, centering and damping forces are implemented
  from scratch using `requestAnimationFrame`
- Nodes are all notes in the vault; edges are `[[wikilink]]` connections
- Currently active note is highlighted in green
- Scroll-to-zoom and drag-to-pan interactions
- Clicking a node navigates to that note and returns to the editor view
- "Reload" button refreshes the graph after vault changes
- Graph toggle button added to the sidebar header (shown when a vault is open)

---

## [0.1.0] — 2026-05-13

Initial working version of the core read/write/sync loop.

### Added

**Vault**
- Open any local directory as a vault via a native OS folder-picker dialog
- `git init` runs automatically if the chosen directory is not already a git repository

**File tree**
- Sidebar lists the vault contents; folders sorted above files
- Hidden files and the `.git` folder are excluded automatically

**Note CRUD**
- Create notes inline from the sidebar with a name input
- Read, write, and delete notes; delete requires confirmation

**Auto-save**
- 1.5 s debounce after the last keystroke before calling `write_note`
- Every save triggers a silent `git add <file>` + `git commit`

**Git sync**
- Sync button runs `git fetch` → `git pull --rebase` → `git push`
- Merge conflicts are detected, the rebase is aborted, and an error is surfaced to the UI

**Backlinks**
- In-memory backlinks index built on vault open with `walkdir`; rebuilt after every save and delete
- Backlinks panel docked below the editor; lists all notes that contain a `[[link]]` pointing to the current one; click any entry to navigate

**Editor**
- CodeMirror 6 editor with Markdown syntax highlighting, line wrapping, and a minimal monospace theme
- `[[wiki-link]]` highlighting via a custom `MatchDecorator` extension
- Ctrl/Cmd+click on a `[[link]]` navigates to the target note; if the note does not exist it is created automatically

**Infrastructure**
- TypeScript IPC wrapper (`api.ts`) — typed wrappers for all Tauri commands
- Svelte stores — reactive state for vault path, file tree, active note, sync status, and backlinks
- Error bar — dismissable inline error messages for all failed operations