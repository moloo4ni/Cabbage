# Cabbage

Une application de notes Markdown local-first utilisant Git comme système de stockage. Un dépôt Git standard sert d'unique source de vérité pour la gestion des connaissances personnelles.

## Core philosophy

* Git comme stockage
* Conception local-first
* Sans cloud et sans serveur
* Uniquement des fichiers Markdown bruts

## Features

* Notes en Markdown
* Synchronisation basée sur Git
* Offline-first
* Application de bureau multiplateforme

## Tech stack

* Tauri
* Rust
* Svelte
* CodeMirror 6
* Git

## Usage concept

* Les notes sont des fichiers Markdown standards (.md).
* Un dossier (vault) est un dépôt Git standard.
* La synchronisation est gérée par des opérations Git standards (push/pull).
* Il n'y a ni comptes utilisateurs ni serveurs centralisés.

## Disclaimer

* Aucune intégration cloud
* Aucun compte utilisateur
* Aucun serveur backend
* Tout est local et strictement basé sur Git