<script lang="ts">
  import { onMount } from 'svelte';
  import { currentVault, syncVault, isSyncing } from './lib/stores/vault';
  // import Editor from './lib/components/Editor.svelte'; // CodeMirror wrapper
  // import Explorer from './lib/components/Explorer.svelte';

  let vaultPath: string | null = null;
  currentVault.subscribe(value => vaultPath = value);
</script>

<main class="flex h-screen w-screen bg-gray-50 text-gray-900">
  <!-- Sidebar / File Explorer -->
  <aside class="w-64 border-r bg-gray-100 flex flex-col">
    <div class="p-4 flex justify-between items-center border-b">
      <span class="font-bold">Cabbage</span>
      <button 
        class="text-sm px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 disabled:opacity-50"
        on:click={syncVault}
        disabled={$isSyncing}
      >
        {$isSyncing ? 'Syncing...' : 'Sync'}
      </button>
    </div>
    <div class="flex-1 overflow-y-auto p-2">
      <!-- File tree stub -->
      <ul>
        <li class="cursor-pointer hover:bg-gray-200 p-1 rounded">Index.md</li>
        <li class="cursor-pointer hover:bg-gray-200 p-1 rounded">Ideas.md</li>
      </ul>
    </div>
  </aside>

  <!-- Editor Pane -->
  <section class="flex-1 flex flex-col">
    <header class="h-12 border-b flex items-center px-4 justify-between">
      <h1 class="text-lg font-medium text-gray-700">Index.md</h1>
      <button class="text-sm text-gray-500 hover:text-gray-900">History</button>
    </header>
    
    <div class="flex-1 overflow-y-auto p-8">
      <!-- CodeMirror Mount Point -->
      <textarea class="w-full h-full bg-transparent resize-none outline-none" placeholder="Start typing..."></textarea>
    </div>
  </section>
</main>

<style>
  :global(body) { margin: 0; font-family: system-ui, sans-serif; }
</style>