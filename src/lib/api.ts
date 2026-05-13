import { invoke } from '@tauri-apps/api/tauri';

export interface FileNode {
    name: string;
    path: string;
    is_dir: boolean;
}

export const api = {
    async openVault(path: string): Promise<string> {
        return invoke('open_vault', { path });
    },

    async listDirectory(subPath: string = ""): Promise<FileNode[]> {
        return invoke('list_directory', { subPath });
    },

    async readNote(relPath: string): Promise<string> {
        return invoke('read_note', { relPath });
    },

    async writeNote(relPath: string, content: string): Promise<void> {
        return invoke('write_note', { relPath, content });
    }
};