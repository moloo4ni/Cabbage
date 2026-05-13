<script lang="ts">
  import './app.css';
  import { api } from './lib/api';
  import { activeVault, fileTree, activeNotePath, isSyncing, backlinks } from './lib/stores';

  let noteContent = '';
  let saveTimer: ReturnType<typeof setTimeout> | null = null;
  let newNoteName = '';
  let showNewNoteInput = false;
  let errorMessage = '';

  // ── Vault ────────────────────────────────────────────────────────────────

  async function handleOpenVault() {
    try {
      errorMessage = '';
      const path = await api.pickAndOpenVault();
      activeVault.set(path);
      await refreshFileTree();
    } catch (e) {
      errorMessage = String(e);
    }
  }

  async function refreshFileTree() {
    const files = await api.listDirectory('');
    fileTree.set(files);
  }

  // ── Note navigation ──────────────────────────────────────────────────────

  async function openFile(relPath: string) {
    try {
      const content = await api.readNote(relPath);
      activeNotePath.set(relPath);
      noteContent = content;

      // Load backlinks for this note
      const noteName = relPath.replace(/\.md$/i, '').split('/').pop() ?? relPath;
      const links = await api.getBacklinks(noteName);
      backlinks.set(links);
    } catch (e) {
      errorMessage = `Failed to open note: ${e}`;
    }
  }

  // ── Auto-save (debounced 1.5 s) ──────────────────────────────────────────

  function handleInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    noteContent = target.value;

    if (!$activeNotePath) return;

    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(async () => {
      try {
        await api.writeNote($activeNotePath!, noteContent);
      } catch (e) {
        errorMessage = `Auto-save failed: ${e}`;
      }
    }, 1500);
  }

  // ── Create note ──────────────────────────────────────────────────────────

  async function handleCreateNote() {
    if (!newNoteName.trim()) return;

    const fileName = newNoteName.endsWith('.md')
      ? newNoteName.trim()
      : `${newNoteName.trim()}.md`;

    try {
      await api.createNote(fileName);
      newNoteName = '';
      showNewNoteInput = false;
      await refreshFileTree();
      await openFile(fileName);
    } catch (e) {
      errorMessage = `Failed to create note: ${e}`;
    }
  }

  // ── Delete note ──────────────────────────────────────────────────────────

  async function handleDeleteNote(relPath: string) {
    if (!confirm(`Delete "${relPath}"?`)) return;

    try {
      await api.deleteNote(relPath);
      if ($activeNotePath === relPath) {
        activeNotePath.set(null);
        noteContent = '';
        backlinks.set([]);
      }
      await refreshFileTree();
    } catch (e) {
      errorMessage = `Failed to delete note: ${e}`;
    }
  }

  // ── Git sync ─────────────────────────────────────────────────────────────

  async function handleSync() {
    isSyncing.set(true);
    errorMessage = '';
    try {
      const result = await api.sync();
      if (!result.success) {
        errorMessage = result.output;
      }
    } catch (e) {
      errorMessage = `Sync failed: ${e}`;
    } finally {
      isSyncing.set(false);
    }
  }
</script>

<main class="layout">

  <!-- Sidebar ---------------------------------------------------------------->
  <aside class="sidebar">
    <div class="sidebar-header">
      <h2>Cabbage</h2>
      {#if $activeVault}
        <button
          class="btn sync-btn"
          on:click={handleSync}
          disabled={$isSyncing}
          title="Sync vault with remote"
        >
          {$isSyncing ? '⟳ Syncing…' : '↑↓ Sync'}
        </button>
      {/if}
    </div>

    {#if !$activeVault}
      <div class="vault-prompt">
        <button on:click={handleOpenVault} class="btn btn-lg">Open Vault</button>
        <p class="hint">Choose a local folder to use as your vault.</p>
      </div>
    {:else}
      <div class="sidebar-toolbar">
        <span class="vault-name" title={$activeVault}>
          {$activeVault.split('/').pop() ?? $activeVault}
        </span>
        <button
          class="icon-btn"
          title="New note"
          on:click={() => (showNewNoteInput = !showNewNoteInput)}
        >+</button>
      </div>

      {#if showNewNoteInput}
        <div class="new-note-form">
          <input
            type="text"
            placeholder="note-name.md"
            bind:value={newNoteName}
            on:keydown={(e) => e.key === 'Enter' && handleCreateNote()}
          />
          <button class="btn" on:click={handleCreateNote}>Create</button>
        </div>
      {/if}

      <div class="file-tree">
        {#each $fileTree as node}
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <div
            class="file-node {node.is_dir ? 'dir' : 'file'} {$activeNotePath === node.path ? 'active' : ''}"
            on:click={() => !node.is_dir && openFile(node.path)}
          >
            <span class="node-icon">{node.is_dir ? '▸' : '·'}</span>
            <span class="node-name">{node.name}</span>
            {#if !node.is_dir}
              <!-- svelte-ignore a11y-click-events-have-key-events -->
              <span
                class="delete-btn"
                title="Delete note"
                on:click|stopPropagation={() => handleDeleteNote(node.path)}
              >✕</span>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </aside>

  <!-- Editor ----------------------------------------------------------------->
  <section class="editor-pane">
    {#if errorMessage}
      <div class="error-bar">{errorMessage} <button on:click={() => (errorMessage = '')}>✕</button></div>
    {/if}

    {#if $activeNotePath}
      <div class="editor-header">
        <span class="path">{$activeNotePath}</span>
      </div>

      <textarea
        class="editor-textarea"
        value={noteContent}
        on:input={handleInput}
        placeholder="Start writing in Markdown…"
      ></textarea>

      {#if $backlinks.length > 0}
        <div class="backlinks-panel">
          <h4>Backlinks</h4>
          <ul>
            {#each $backlinks as link}
              <li>
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <span class="backlink-item" on:click={() => openFile(link)}>{link}</span>
              </li>
            {/each}
          </ul>
        </div>
      {/if}
    {:else}
      <div class="empty-state">
        <p>Select a note or create a new one.</p>
      </div>
    {/if}
  </section>

</main>

<style>
  /* ── Layout ──────────────────────────────────────────────────────────────── */
  .layout { display: flex; height: 100vh; width: 100vw; overflow: hidden; }

  /* ── Sidebar ─────────────────────────────────────────────────────────────── */
  .sidebar {
    width: 260px;
    background: var(--sidebar-bg);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .sidebar-header {
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-shrink: 0;
  }
  .sidebar-header h2 { margin: 0; font-size: 15px; color: var(--accent); font-weight: 700; }

  .vault-prompt {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 24px;
  }
  .hint { font-size: 13px; color: var(--text-muted); text-align: center; }

  .sidebar-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    font-size: 12px;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border-color);
    flex-shrink: 0;
  }
  .vault-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 160px;
  }
  .icon-btn {
    background: none;
    border: none;
    font-size: 18px;
    color: var(--text-muted);
    cursor: pointer;
    line-height: 1;
    padding: 0 4px;
  }
  .icon-btn:hover { color: var(--text-main); }

  .new-note-form {
    display: flex;
    gap: 6px;
    padding: 8px 12px;
    border-bottom: 1px solid var(--border-color);
    flex-shrink: 0;
  }
  .new-note-form input {
    flex: 1;
    padding: 4px 8px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-size: 13px;
  }

  .file-tree { padding: 6px 8px; flex: 1; overflow-y: auto; }

  .file-node {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 8px;
    margin-bottom: 1px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 13px;
    color: var(--text-muted);
    position: relative;
  }
  .file-node:hover { background: rgba(0,0,0,0.05); color: var(--text-main); }
  .file-node.active { background: var(--border-color); color: var(--text-main); font-weight: 500; }
  .file-node.dir { cursor: default; font-weight: 600; }

  .node-icon { font-size: 10px; flex-shrink: 0; }
  .node-name { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

  .delete-btn {
    display: none;
    font-size: 11px;
    color: #aaa;
    flex-shrink: 0;
  }
  .file-node:hover .delete-btn { display: inline; }
  .delete-btn:hover { color: #e00; }

  /* ── Editor ──────────────────────────────────────────────────────────────── */
  .editor-pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: var(--bg-color);
    overflow: hidden;
  }

  .error-bar {
    background: #fee2e2;
    color: #991b1b;
    font-size: 13px;
    padding: 8px 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-shrink: 0;
  }
  .error-bar button {
    background: none;
    border: none;
    cursor: pointer;
    color: inherit;
    font-size: 14px;
  }

  .editor-header {
    height: 44px;
    padding: 0 20px;
    border-bottom: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }
  .path { font-size: 13px; color: var(--text-muted); }

  .editor-textarea {
    flex: 1;
    width: 100%;
    padding: 32px 40px;
    border: none;
    resize: none;
    outline: none;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 15px;
    line-height: 1.7;
    box-sizing: border-box;
    background: var(--bg-color);
    color: var(--text-main);
  }

  .backlinks-panel {
    border-top: 1px solid var(--border-color);
    padding: 12px 20px;
    flex-shrink: 0;
    background: var(--sidebar-bg);
  }
  .backlinks-panel h4 {
    margin: 0 0 8px;
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }
  .backlinks-panel ul { margin: 0; padding: 0; list-style: none; }
  .backlinks-panel li { margin-bottom: 4px; }
  .backlink-item {
    font-size: 13px;
    color: var(--accent);
    cursor: pointer;
    text-decoration: underline;
  }
  .backlink-item:hover { opacity: 0.8; }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 14px;
  }

  /* ── Buttons ─────────────────────────────────────────────────────────────── */
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
  .btn:hover:not(:disabled) { opacity: 0.85; }
  .btn:disabled { opacity: 0.5; cursor: not-allowed; }
  .btn-lg { padding: 10px 20px; font-size: 15px; }

  .sync-btn { font-size: 12px; padding: 4px 10px; }
</style>
