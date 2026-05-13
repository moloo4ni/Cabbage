# Changelog

All notable changes to Cabbage are documented in this file.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Planned
- Note history view — per-file `git log`, diff viewer, version restore
- Graph view — visual node graph of backlinks (D3.js / canvas)
- Native Rust Git bindings — replace shell subprocess wrappers with `gitoxide` or `libgit2`

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
