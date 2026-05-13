# Cabbage

Cabbage is a local-first, cross-platform desktop application designed for personal knowledge management. Rather than relying on proprietary database formats or centralized cloud servers, it uses a standard Git repository as its single source of truth and pure Markdown files for data storage.

## Core Philosophy

The architecture is built around the concept of maximum data portability, transparency, and offline availability. By leveraging Git for version control and synchronization, users retain complete ownership of their data. There are no background sync services running on third-party infrastructure, no user accounts, and zero vendor lock-in. If the application is ever uninstalled, the user's knowledge base remains fully accessible as a standard directory of text files with a complete Git revision history.

## How It Works

In Cabbage, a "vault" is simply a local directory on the user's file system initialized as a Git repository. Notes are created as standard Markdown (`.md`) files within this directory. 

As the user edits notes, the application handles versioning by making automatic, silent Git commits in the background. Synchronization across multiple devices is achieved entirely through standard Git network operations (fetch, pull with rebase, and push) against any remote repository supporting SSH or HTTPS protocols. This allows users to sync their vaults using GitHub, GitLab, or any self-hosted Git server. 

The application is strictly offline-first. Network connectivity is completely optional and is only required during explicit synchronization events triggered by the user.

## Technical Architecture

The application is structured as a decoupled system utilizing modern desktop technologies:

* **Presentation Layer:** The user interface is built with Svelte and utilizes CodeMirror 6 for a robust, syntax-aware Markdown editing experience. The frontend holds no persistent state and acts only as a rendering engine.
* **Bridge Layer:** The frontend runs inside a secure webview managed by Tauri. All interactions between the user interface and the underlying system occur via Tauri's Inter-Process Communication (IPC) commands.
* **Core Layer:** The backend logic, file system operations, and Git integrations are written in Rust. Upon opening a vault, the Rust core scans the directory and builds an in-memory graph index to resolve note links instantly without requiring a persistent database like SQLite.

Currently, Git operations are executed via safe shell subprocess wrappers, with an architectural roadmap aimed at migrating to native Rust Git bindings (`gitoxide` or `libgit2`) in future releases.

## Local Development Setup

To build and run Cabbage locally, ensure you have the Rust toolchain, Node.js, pnpm, and standard Tauri system dependencies installed.

Clone the repository and install the frontend dependencies:

```bash
pnpm install
```

Start the application in development mode with hot-module replacement:

```bash
pnpm tauri dev
```

To compile a standalone release binary for your current operating system:

```bash
pnpm tauri build
```

## Disclaimer

Cabbage does not provide any proprietary cloud infrastructure. It does not track user metrics, require registration, or communicate with any external servers other than the explicit Git remotes configured by the user. Everything is handled locally on the host machine.