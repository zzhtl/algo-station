<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { renderMarkdown } from '$lib/utils/markdown';
  import 'katex/dist/katex.min.css';

  export let source: string;

  let container: HTMLElement;
  let html = '';

  $: void update(source);

  async function update(src: string) {
    html = await renderMarkdown(src);
    await tick();
    linkProblemReferences();
    await renderMermaid();
    renderMath();
  }

  function linkProblemReferences() {
    if (!container) return;
    const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT, {
      acceptNode(node) {
        if (!node.nodeValue || !/#\d{1,5}/.test(node.nodeValue)) return NodeFilter.FILTER_REJECT;
        const p = node.parentElement;
        if (!p) return NodeFilter.FILTER_REJECT;
        if (['A', 'CODE', 'PRE', 'SCRIPT', 'STYLE'].includes(p.tagName)) return NodeFilter.FILTER_REJECT;
        return NodeFilter.FILTER_ACCEPT;
      }
    });

    const nodes: Text[] = [];
    let n: Node | null;
    while ((n = walker.nextNode())) nodes.push(n as Text);

    for (const node of nodes) {
      const text = node.nodeValue ?? '';
      const frag = document.createDocumentFragment();
      let lastIndex = 0;
      const re = /#(\d{1,5})(?=$|[\s,，、.。:：)）])/g;
      let match: RegExpExecArray | null;
      while ((match = re.exec(text)) !== null) {
        frag.append(document.createTextNode(text.slice(lastIndex, match.index)));
        const a = document.createElement('a');
        a.href = `/problems/${match[1]}`;
        a.textContent = match[0];
        a.className = 'problem-ref';
        frag.append(a);
        lastIndex = match.index + match[0].length;
      }
      frag.append(document.createTextNode(text.slice(lastIndex)));
      node.parentNode?.replaceChild(frag, node);
    }
  }

  async function renderMermaid() {
    if (!container) return;
    const nodes = container.querySelectorAll<HTMLDivElement>('.mermaid-source');
    if (nodes.length === 0) return;

    const mermaid = (await import('mermaid')).default;
    mermaid.initialize({
      startOnLoad: false,
      theme: 'dark',
      themeVariables: {
        background: '#11141b',
        primaryColor: '#161a23',
        primaryTextColor: '#e6e8ee',
        primaryBorderColor: '#3d4a7a',
        lineColor: '#7c9cff',
        secondaryColor: '#1f2937',
        tertiaryColor: '#0b0d12',
        fontFamily: 'ui-sans-serif, system-ui, sans-serif'
      },
      flowchart: { curve: 'basis', htmlLabels: true },
      securityLevel: 'loose'
    });

    let i = 0;
    for (const node of Array.from(nodes)) {
      const raw = node.getAttribute('data-mermaid') ?? '';
      const code = decode(raw);
      const id = `mmd-${Date.now()}-${i++}`;
      try {
        const { svg } = await mermaid.render(id, code);
        node.innerHTML = svg;
        node.classList.remove('mermaid-source');
      } catch (e) {
        node.innerHTML = `<pre class="text-hard text-xs">Mermaid 渲染失败: ${String(e)}</pre>`;
      }
    }
  }

  function renderMath() {
    if (!container) return;
    import('katex').then(({ default: katex }) => {
      const textNodes: Text[] = [];
      const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT, {
        acceptNode(node) {
          if (!node.nodeValue) return NodeFilter.FILTER_REJECT;
          const p = node.parentElement;
          if (!p) return NodeFilter.FILTER_REJECT;
          if (['CODE', 'PRE', 'SCRIPT', 'STYLE'].includes(p.tagName)) return NodeFilter.FILTER_REJECT;
          if (/\$\$[^$]+\$\$|\$[^$\n]+\$/.test(node.nodeValue)) return NodeFilter.FILTER_ACCEPT;
          return NodeFilter.FILTER_REJECT;
        }
      });
      let n: Node | null;
      while ((n = walker.nextNode())) textNodes.push(n as Text);

      for (const node of textNodes) {
        const original = node.nodeValue ?? '';
        const span = document.createElement('span');
        span.innerHTML = original
          .replace(/\$\$([^$]+)\$\$/g, (_, expr) => {
            try {
              return katex.renderToString(expr, { displayMode: true, throwOnError: false });
            } catch {
              return _;
            }
          })
          .replace(/\$([^$\n]+)\$/g, (_, expr) => {
            try {
              return katex.renderToString(expr, { displayMode: false, throwOnError: false });
            } catch {
              return _;
            }
          });
        node.parentNode?.replaceChild(span, node);
      }
    });
  }

  function decode(s: string): string {
    return s
      .replace(/&amp;/g, '&')
      .replace(/&lt;/g, '<')
      .replace(/&gt;/g, '>')
      .replace(/&quot;/g, '"')
      .replace(/&#39;/g, "'");
  }

  onMount(() => update(source));
</script>

<div bind:this={container} class="prose-dark">
  {@html html}
</div>

<style>
  :global(.code-block) {
    margin: 1rem 0;
    border-radius: 0.5rem;
    overflow: hidden;
    border: 1px solid #252b39;
  }
  :global(.code-block pre) {
    margin: 0 !important;
    padding: 1rem !important;
    overflow-x: auto;
    font-size: 0.85rem;
    line-height: 1.6;
  }
  :global(.mermaid-container svg) {
    max-width: 100%;
    height: auto;
  }
  :global(.problem-ref) {
    display: inline-flex;
    align-items: center;
    border: 1px solid #252b39;
    border-radius: 0.375rem;
    background: #161a23;
    padding: 0 0.35rem;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 0.9em;
    color: #7c9cff !important;
    text-decoration: none !important;
  }
  :global(.problem-ref:hover) {
    border-color: rgba(124, 156, 255, 0.5);
    background: rgba(124, 156, 255, 0.12);
  }
</style>
