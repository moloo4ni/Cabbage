import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

export const currentVault = writable<string | null>(null);
export const currentNote = writable<string | null>(null);
export const isSyncing = writable<boolean>(false);

export async function saveNote(path: string, content: string) {
    try {
        await invoke('write_note', { path, content });
    } catch (error) {
        console.error("Failed to save note:", error);
    }
}

export async function syncVault() {
    isSyncing.set(true);
    try {
        const result = await invoke('sync');
        console.log("Sync success:", result);
    } catch (error) {
        console.error("Sync failed (merge conflict?):", error);
        // Тут триггерим UI решения конфликта
    } finally {
        isSyncing.set(false);
    }
}