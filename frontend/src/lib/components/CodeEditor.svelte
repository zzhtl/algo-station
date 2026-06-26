<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import { EditorState, Compartment } from '@codemirror/state';
  import {
    EditorView,
    keymap,
    lineNumbers,
    highlightActiveLine,
    highlightActiveLineGutter,
    placeholder as cmPlaceholder
  } from '@codemirror/view';
  import { history, defaultKeymap, historyKeymap, indentWithTab } from '@codemirror/commands';
  import {
    indentUnit,
    syntaxHighlighting,
    HighlightStyle,
    bracketMatching,
    StreamLanguage
  } from '@codemirror/language';
  import { tags as t } from '@lezer/highlight';
  import { rust } from '@codemirror/lang-rust';
  import { python } from '@codemirror/lang-python';
  import { go } from '@codemirror/legacy-modes/mode/go';

  export let value = '';
  export let lang: 'go' | 'rust' | 'python' = 'go';
  export let placeholder = '';

  const dispatch = createEventDispatcher<{ change: string }>();
  let host: HTMLDivElement;
  let view: EditorView | null = null;
  let internalValue = value;
  const langCompartment = new Compartment();

  function langExt(l: string) {
    if (l === 'rust') return rust();
    if (l === 'python') return python();
    return StreamLanguage.define(go);
  }

  // 中性色语法高亮：亮暗主题下都可读（不依赖背景）。
  const highlight = HighlightStyle.define([
    { tag: t.keyword, color: '#c792ea' },
    { tag: [t.function(t.variableName), t.labelName, t.propertyName], color: '#82aaff' },
    { tag: [t.string, t.inserted, t.special(t.string)], color: '#1a9f6b' },
    { tag: [t.number, t.bool, t.null, t.atom], color: '#e5734d' },
    { tag: [t.comment, t.meta], color: '#7a8699', fontStyle: 'italic' },
    { tag: [t.typeName, t.className, t.namespace], color: '#d9a93e' },
    { tag: t.operator, color: '#3a9ec9' }
  ]);

  // 编辑器底色透明，跟随外层容器（容器用 CSS 变量随主题切换）。
  const theme = EditorView.theme({
    '&': { backgroundColor: 'transparent', height: '100%', fontSize: '13px' },
    '&.cm-focused': { outline: 'none' },
    '.cm-scroller': {
      fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Consolas, monospace',
      lineHeight: '1.65',
      overflow: 'auto'
    },
    '.cm-content': { padding: '12px 0' },
    '.cm-gutters': {
      backgroundColor: 'transparent',
      borderRight: '1px solid rgba(128,128,128,0.15)',
      color: 'rgba(128,128,128,0.55)'
    },
    '.cm-activeLine': { backgroundColor: 'rgba(124,156,255,0.06)' },
    '.cm-activeLineGutter': { backgroundColor: 'rgba(124,156,255,0.08)' },
    '.cm-cursor': { borderLeftColor: '#7c9cff' },
    '.cm-selectionBackground, &.cm-focused .cm-selectionBackground': {
      backgroundColor: 'rgba(124,156,255,0.22)'
    }
  });

  function onDocChanged(v: string) {
    internalValue = v;
    value = v;
    dispatch('change', v);
  }

  // 外部改 value（重置/填入参考解法/切题）时同步进编辑器，guard 防回环。
  $: if (view && value !== internalValue) {
    internalValue = value;
    view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: value } });
  }

  // 切换语言时重配语言扩展。
  $: if (view) {
    view.dispatch({ effects: langCompartment.reconfigure(langExt(lang)) });
  }

  onMount(() => {
    view = new EditorView({
      parent: host,
      state: EditorState.create({
        doc: value,
        extensions: [
          lineNumbers(),
          highlightActiveLineGutter(),
          highlightActiveLine(),
          history(),
          bracketMatching(),
          indentUnit.of('    '),
          keymap.of([indentWithTab, ...defaultKeymap, ...historyKeymap]),
          langCompartment.of(langExt(lang)),
          syntaxHighlighting(highlight),
          cmPlaceholder(placeholder),
          theme,
          EditorView.updateListener.of((u) => {
            if (u.docChanged) onDocChanged(u.state.doc.toString());
          })
        ]
      })
    });
  });

  onDestroy(() => view?.destroy());
</script>

<div
  bind:this={host}
  class="cm-host min-h-[22rem] resize-y overflow-hidden rounded-lg border border-bg-border bg-bg-soft"
></div>
