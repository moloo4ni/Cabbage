# Cabbage

Una aplicación de notas Markdown local-first que utiliza Git como sistema de almacenamiento. Un repositorio Git estándar sirve como la única fuente de verdad para la gestión del conocimiento personal.

## Core philosophy

* Git como almacenamiento
* Diseño local-first
* Sin nube y sin servidor
* Solo archivos Markdown de texto plano

## Features

* Notas en Markdown
* Sincronización basada en Git
* Offline-first
* Aplicación de escritorio multiplataforma

## Tech stack

* Tauri
* Rust
* Svelte
* CodeMirror 6
* Git

## Usage concept

* Las notas son archivos Markdown estándar (.md).
* Una carpeta (vault) es un repositorio Git estándar.
* La sincronización se maneja a través de operaciones estándar de Git (push/pull).
* No hay cuentas de usuario ni servidores centralizados.

## Disclaimer

* Sin integración en la nube
* Sin cuentas de usuario
* Sin servidor backend
* Todo es local y estrictamente basado en Git