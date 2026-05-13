import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default defineConfig(async () => ({
  plugins: [svelte()],
  // Tauri ожидает фиксированный порт, чтобы не было проблем с CORS
  server: {
    port: 1420,
    strictPort: true,
  },
  // Используем относительные пути
  base: "./",
  clearScreen: false,
}));