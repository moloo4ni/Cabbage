import { invoke } from '@tauri-apps/api/tauri';

export interface FileNode {
    name: string;
    path: string;
    is_dir: boolean;
}

export interface GitResult {
    success: boolean;
    output: string;
}

export const api = {
    // ── Vault ──────────────────────────────────────────────────────────────

    /** Opens an OS folder-picker dialog and sets the active vault. */
    async pickAndOpenVault(): Promise<string> {
        return invoke('pick_and_open_vault');
    },

    /** Opens a vault at a known path (e.g. recently used). */
    async openVault(path: string): Promise<string> {
        return invoke('open_vault', { path });
    },

    // ── File tree ──────────────────────────────────────────────────────────

    async listDirectory(subPath: string = ''): Promise<FileNode[]> {
        return invoke('list_directory', { subPath });
    },

    // ── Note CRUD ──────────────────────────────────────────────────────────

    async readNote(relPath: string): Promise<string> {
        return invoke('read_note', { relPath });
    },

    async writeNote(relPath: string, content: string): Promise<void> {
        return invoke('write_note', { relPath, content });
    },

    async createNote(relPath: string): Promise<void> {
        return invoke('create_note', { relPath });
    },

    async deleteNote(relPath: string): Promise<void> {
        return invoke('delete_note', { relPath });
    },

    // ── Knowledge graph ────────────────────────────────────────────────────

    /** Returns paths of all notes that link to `noteName` via [[noteName]]. */
    async getBacklinks(noteName: string): Promise<string[]> {
        return invoke('get_backlinks', { noteName });
    },

    // ── Git sync ───────────────────────────────────────────────────────────

    async sync(): Promise<GitResult> {
        return invoke('sync');
    },
};
