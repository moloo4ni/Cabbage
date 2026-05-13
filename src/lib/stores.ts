import { writable } from 'svelte/store';
import type { FileNode } from './api';

// Путь к текущему открытому хранилищу (vault)
export const activeVault = writable<string | null>(null);

// Относительный путь текущей открытой заметки (например, "Ideas/Project.md")
export const activeNotePath = writable<string | null>(null);

// Дерево файлов для сайдбара
export const fileTree = writable<FileNode[]>([]);