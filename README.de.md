## ![Cabbage](assets/banner.png)

Eine Local-First Markdown-Notiz-Anwendung, die Git als Speicher nutzt. Ein Standard-Git-Repository dient als einzige Datenquelle für das persönliche Wissensmanagement.

## Core philosophy

* Git als Speicher
* Local-First-Design
* Keine Cloud und kein Server
* Ausschließlich reine Markdown-Dateien

## Features

* Markdown-Notizen
* Git-basierte Synchronisation
* Offline-First
* Plattformübergreifende Desktop-Anwendung

## Tech stack

* Tauri
* Rust
* Svelte
* CodeMirror 6
* Git

## Usage concept

* Notizen sind Standard-Markdown-Dateien (.md).
* Ein Ordner (Vault) ist ein Standard-Git-Repository.
* Die Synchronisation erfolgt über Standard-Git-Operationen (push/pull).
* Es gibt keine Benutzerkonten oder zentralisierten Server.

## Disclaimer

* Keine Cloud-Integration
* Keine Benutzerkonten
* Kein Backend-Server
* Alles ist lokal und streng Git-basiert