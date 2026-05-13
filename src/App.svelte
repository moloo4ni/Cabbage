<script lang="ts">
  import './app.css';
  import { api } from './lib/api';
  import { activeVault, fileTree, activeNotePath } from './lib/stores';

  let noteContent = "";

  // Открытие хранилища (вызов Rust)
  async function handleOpenVault() {
    try {
      // Передаем пустую строку, чтобы Rust открыл диалог выбора папки
      // (пока в Rust мы не реализовали диалог, но логика будет такой)
      const path = await api.openVault("C:/Vault"); // Временно хардкодим путь для теста! 
      activeVault.set(path);
      
      // Загружаем файлы
      const files = await api.listDirectory("");
      fileTree.set(files);
    } catch (e) {
      console.error("Failed to open vault:", e);
    }
  }

  // Клик по файлу в сайдбаре
  async function openFile(relPath: string) {
    try {
      const content = await api.readNote(relPath);
      activeNotePath.set(relPath);
      noteContent = content;
    } catch (e) {
      console.error("Failed to read note:", e);
    }
  }

  // Сохранение (будет вызываться на каждое изменение с задержкой)
  async function handleInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    noteContent = target.value;
    
    if ($activeNotePath) {
      // Тут в будущем добавим debounce (задержку в 1-2 сек)
      await api.writeNote($activeNotePath, noteContent);
    }
  }
</script>

<main class="layout">
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-header">
      <h2>Cabbage</h2>
      {#if !$activeVault}
        <button on:click={handleOpenVault} class="btn">Open Vault</button>
      {/if}
    </div>

    {#if $activeVault}
      <div class="file-tree">
        {#each $fileTree as node}
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <div 
            class="file-node {node.is_dir ? 'dir' : 'file'} {$activeNotePath === node.path ? 'active' : ''}"
            on:click={() => !node.is_dir && openFile(node.path)}
          >
            {node.is_dir ? '📁' : '📄'} {node.name}
          </div>
        {/each}
      </div>
    {/if}
  </aside>

  <!-- Editor Pane -->
  <section class="editor-pane">
    {#if $activeNotePath}
      <div class="editor-header">
        <span class="path">{$activeNotePath}</span>
        <button class="btn sync-btn">Sync</button>
      </div>
      <textarea 
        class="editor-textarea" 
        value={noteContent} 
        on:input={handleInput}
        placeholder="Start writing in Markdown..."
      ></textarea>
    {:else}
      <div class="empty-state">
        <p>Select a note or create a new one.</p>
      </div>
    {/if}
  </section>
</main>

<style>
  .layout { display: flex; height: 100vh; width: 100vw; }
  
  .sidebar {
    width: 260px;
    background: var(--sidebar-bg);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
  }

  .sidebar-header {
    padding: 16px;
    border-bottom: 1px solid var(--border-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .sidebar-header h2 { margin: 0; font-size: 16px; color: var(--accent); }

  .file-tree { padding: 8px; flex: 1; overflow-y: auto; }
  
  .file-node {
    padding: 6px 8px;
    margin-bottom: 2px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    color: var(--text-muted);
  }

  .file-node:hover { background: #e5e7eb; color: var(--text-main); }
  .file-node.active { background: var(--border-color); color: var(--text-main); font-weight: 500; }
  .file-node.dir { font-weight: bold; pointer-events: none; } /* Пока папки нельзя кликать */

  .editor-pane { flex: 1; display: flex; flex-direction: column; background: var(--bg-color); }
  
  .editor-header {
    height: 50px;
    padding: 0 20px;
    border-bottom: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  
  .path { font-size: 14px; color: var(--text-muted); }
  
  .editor-textarea {
    flex: 1;
    width: 100%;
    padding: 40px;
    border: none;
    resize: none;
    outline: none;
    font-family: monospace; /* Временно, пока не подключим CodeMirror */
    font-size: 16px;
    line-height: 1.6;
    box-sizing: border-box;
  }

  .empty-state {
    flex: 1; display: flex; align-items: center; justify-content: center; color: var(--text-muted);
  }

  .btn {
    background: var(--accent);
    color: white;
    border: none;
    padding: 6px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
  }
  .btn:hover { opacity: 0.9; }
</style>