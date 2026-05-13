import {
  ViewPlugin,
  Decoration,
  MatchDecorator,
  EditorView,
  type DecorationSet,
  type ViewUpdate,
} from '@codemirror/view';

const WIKILINK_RE = /\[\[([^\[\]\n]+)\]\]/g;

const decorator = new MatchDecorator({
  regexp: WIKILINK_RE,
  decoration: (match) =>
    Decoration.mark({
      class: 'cm-wikilink',
      attributes: { 'data-target': match[1].trim() },
    }),
});

/**
 * Highlights [[wiki-links]] and navigates to them on Ctrl/Cmd+click.
 */
export function wikilinkPlugin(onNavigate: (target: string) => void) {
  return [
    ViewPlugin.fromClass(
      class {
        decorations: DecorationSet;

        constructor(view: EditorView) {
          this.decorations = decorator.createDeco(view);
        }

        update(update: ViewUpdate) {
          this.decorations = decorator.updateDeco(update, this.decorations);
        }
      },
      { decorations: (v) => v.decorations }
    ),

    EditorView.domEventHandlers({
      click(event: MouseEvent) {
        const el = (event.target as HTMLElement).closest<HTMLElement>('.cm-wikilink');
        if (!el) return false;
        const target = el.dataset.target;
        if (target && (event.ctrlKey || event.metaKey)) {
          onNavigate(target);
          return true;
        }
        return false;
      },
    }),
  ];
}

export const wikilinkTheme = EditorView.baseTheme({
  '.cm-wikilink': {
    color: 'var(--accent, #2563eb)',
    textDecoration: 'underline',
    textUnderlineOffset: '2px',
    cursor: 'pointer',
  },
  '.cm-wikilink:hover': {
    opacity: '0.8',
  },
});
