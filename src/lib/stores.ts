import { writable } from 'svelte/store';
import type { FileNode } from './api';

/** Absolute path to the currently open vault directory. */
export const activeVault = writable<string | null>(null);

/** Vault-relative path of the note currently open in the editor. */
export const activeNotePath = writable<string | null>(null);

/** Flat list of file/folder nodes shown in the sidebar. */
export const fileTree = writable<FileNode[]>([]);

/** True while a git sync operation is in progress. */
export const isSyncing = writable<boolean>(false);

/** Paths of notes that link to the currently active note (backlinks panel). */
export const backlinks = writable<string[]>([]);
