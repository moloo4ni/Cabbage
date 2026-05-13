# Cabbage

Cabbage is a local-first, cross-platform desktop application for personal knowledge management. It stores all notes as plain Markdown files inside a Git repository, giving you versioning, offline use, and full data portability — no proprietary database, no cloud account, no vendor lock-in.

## How It Works

A **vault** is simply a local directory on your file system. Open any folder in Cabbage and it becomes your vault — if it is not already a Git repository, Cabbage runs `git init` automatically.

Notes are standard `.md` files. As you edit, Cabbage saves your changes and silently commits them to the local repository. Syncing with another machine is a regular `git push` and `git pull --rebase` against any remote you configure (GitHub, GitLab, a self-hosted server — anything that speaks Git over SSH or HTTPS).

## Current State

The core read/write/sync loop is working:

- Open a vault via a native folder-picker dialog
- Browse the file tree in the sidebar
- Create and delete notes
- Edit notes with CodeMirror 6 — Markdown syntax highlighting, line wrapping, minimal theme
- `[[wiki-links]]` highlighted inline; Ctrl/Cmd+click navigates to the linked note (creates it if it does not exist)
- Auto-save with a 1.5 s debounce — every save triggers an automatic local Git commit
- Sync button runs `git fetch` + `git pull --rebase` + `git push`
- In-memory backlinks index — notes that link to the current note are shown in the backlinks panel
- Inline error bar for failed operations

## Architecture

The application is structured as a decoupled system:

- **Frontend (Svelte):** Handles UI rendering and user interactions. Holds no persistent state — everything is fetched from the Rust core via IPC. Editor is CodeMirror 6 with a custom `[[wiki-link]]` extension.
- **Bridge (Tauri IPC):** Secure communication channel between the Svelte webview and the native system.
- **Core (Rust):** File system operations, Git commands (via shell subprocess wrappers), and an in-memory backlinks index built with `walkdir` on vault open.

Git operations currently use shell subprocess wrappers. The roadmap includes migrating to native Rust Git bindings (`gitoxide` or `libgit2`).

## Roadmap

- [x] CodeMirror 6 editor with Markdown syntax highlighting
- [x] `[[wiki-link]]` highlighting and click-to-navigate
- [ ] Note history view (per-file `git log` + diff + restore)
- [ ] Graph view (visual node graph of backlinks)
- [ ] Native Rust Git bindings (replace shell subprocess wrappers)

## Local Development

**Prerequisites:** Rust toolchain, Node.js (v18+), pnpm, Git, and the [Tauri system dependencies](https://tauri.app/v1/guides/getting-started/prerequisites) for your OS.

```bash
git clone https://gitlab.com/moloo4ni/cabbage.git
cd cabbage
pnpm install
pnpm tauri dev
```

**Build a release binary:**

```bash
pnpm tauri build
# Output: src-tauri/target/release/bundle/
```

## Disclaimer

Cabbage does not track user metrics, require registration, or communicate with any external servers other than the Git remotes you configure yourself. Everything runs locally on your machine.
