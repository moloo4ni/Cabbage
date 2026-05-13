<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import {
    EditorView,
    keymap,
    highlightActiveLine,
  } from '@codemirror/view';
  import { EditorState } from '@codemirror/state';
  import {
    defaultKeymap,
    history,
    historyKeymap,
    indentWithTab,
  } from '@codemirror/commands';
  import { markdown } from '@codemirror/lang-markdown';
  import { wikilinkPlugin, wikilinkTheme } from './wikilink';

  export let value: string = '';
  export let onNavigate: (target: string) => void = () => {};

  const dispatch = createEventDispatcher<{ input: string }>();

  let container: HTMLDivElement;
  let view: EditorView;
  let suppressing = false;

  const editorTheme = EditorView.theme({
    '&': {
      height: '100%',
      fontSize: '15px',
      fontFamily: "'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace",
    },
    '&.cm-focused': { outline: 'none' },
    '.cm-scroller': {
      overflow: 'auto',
      padding: '32px 40px',
      lineHeight: '1.7',
      boxSizing: 'border-box',
    },
    '.cm-content': {
      caretColor: '#374151',
      maxWidth: '720px',
    },
    '.cm-gutters': {
      backgroundColor: 'transparent',
      border: 'none',
    },
    '.cm-activeLine': { backgroundColor: 'rgba(0,0,0,0.03)' },
    '.cm-cursor': { borderLeftColor: '#374151' },
    '.cm-line': { padding: '0' },
  });

  function makeState(doc: string): EditorState {
    return EditorState.create({
      doc,
      extensions: [
        history(),
        keymap.of([...defaultKeymap, ...historyKeymap, indentWithTab]),
        highlightActiveLine(),
        markdown(),
        wikilinkPlugin(onNavigate),
        wikilinkTheme,
        editorTheme,
        EditorView.lineWrapping,
        EditorView.updateListener.of(update => {
          if (update.docChanged && !suppressing) {
            dispatch('input', update.state.doc.toString());
          }
        }),
      ],
    });
  }

  onMount(() => {
    view = new EditorView({ state: makeState(value), parent: container });
  });

  onDestroy(() => view?.destroy());

  // Sync external value changes (switching notes) without triggering 'input'
  $: if (view) {
    const current = view.state.doc.toString();
    if (current !== value) {
      suppressing = true;
      view.dispatch({
        changes: { from: 0, to: view.state.doc.length, insert: value },
      });
      suppressing = false;
    }
  }
</script>

<div bind:this={container} class="editor-wrap"></div>

<style>
  .editor-wrap {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  .editor-wrap :global(.cm-editor) {
    height: 100%;
  }
</style>
