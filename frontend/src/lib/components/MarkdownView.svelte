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
    enhanceCodeBlocks();
    await renderMermaid();
    renderMath();
  }

  // 给高亮后的代码块加一条工具栏（语言标签 + 一键复制）。
  function enhanceCodeBlocks() {
    if (!container) return;
    for (const block of Array.from(container.querySelectorAll<HTMLElement>('.code-block'))) {
      if (block.querySelector('.code-block-bar')) continue;
      const lang = block.getAttribute('data-lang') ?? '';
      const bar = document.createElement('div');
      bar.className = 'code-block-bar';
      const label = document.createElement('span');
      label.className = 'code-block-lang';
      label.textContent = lang;
      const btn = document.createElement('button');
      btn.type = 'button';
      btn.className = 'code-copy-btn';
      btn.textContent = '复制';
      btn.addEventListener('click', () => {
        const code = block.querySelector('code')?.textContent ?? '';
        navigator.clipboard?.writeText(code).then(() => {
          btn.textContent = '已复制';
          setTimeout(() => (btn.textContent = '复制'), 1500);
        });
      });
      bar.append(label, btn);
      block.prepend(bar);
    }
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
    const dark = document.documentElement.classList.contains('dark');
    mermaid.initialize({
      startOnLoad: false,
      theme: dark ? 'dark' : 'default',
      themeVariables: dark
        ? {
            background: '#11141b',
            primaryColor: '#161a23',
            primaryTextColor: '#e6e8ee',
            primaryBorderColor: '#3d4a7a',
            lineColor: '#7c9cff',
            secondaryColor: '#1f2937',
            tertiaryColor: '#0b0d12',
            fontFamily: 'ui-sans-serif, system-ui, sans-serif'
          }
        : {
            background: '#ffffff',
            primaryColor: '#eef2ff',
            primaryTextColor: '#1e2530',
            primaryBorderColor: '#3b5bdb',
            lineColor: '#3b5bdb',
            secondaryColor: '#e2e8f0',
            tertiaryColor: '#f8fafc',
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

  onMount(() => {
    update(source);
    const onThemeChange = () => update(source);
    window.addEventListener('themechange', onThemeChange);
    return () => window.removeEventListener('themechange', onThemeChange);
  });
</script>

<div bind:this={container} class="prose-dark">
  {@html html}
</div>

<style>
  /* 代码块恒为暗色面板（shiki github-dark-dimmed），工具栏/行号用浅色叠加，亮暗主题下都贴合。 */
  :global(.code-block) {
    position: relative;
    margin: 1rem 0;
    border-radius: 0.5rem;
    overflow: hidden;
    border: 1px solid rgb(var(--c-bg-border));
  }
  :global(.code-block pre) {
    margin: 0 !important;
    padding: 1rem !important;
    overflow-x: auto;
    font-size: 0.85rem;
    line-height: 1.6;
  }
  :global(.code-block-bar) {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.3rem 0.75rem;
    background: rgba(255, 255, 255, 0.04);
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }
  :global(.code-block-lang) {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 0.68rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: rgba(255, 255, 255, 0.4);
  }
  :global(.code-copy-btn) {
    font-size: 0.7rem;
    color: rgba(255, 255, 255, 0.6);
    padding: 0.1rem 0.55rem;
    border-radius: 0.25rem;
    border: 1px solid rgba(255, 255, 255, 0.15);
    background: transparent;
    cursor: pointer;
    transition: all 0.15s;
  }
  :global(.code-copy-btn:hover) {
    color: rgb(var(--c-accent));
    border-color: rgb(var(--c-accent) / 0.5);
  }
  /* 行号：shiki 把每行包成 .line，用 counter 渲染 */
  :global(.code-block .shiki code) {
    counter-reset: line;
  }
  :global(.code-block .shiki .line) {
    counter-increment: line;
  }
  :global(.code-block .shiki .line::before) {
    content: counter(line);
    display: inline-block;
    width: 1.4rem;
    margin-right: 1rem;
    text-align: right;
    color: rgba(255, 255, 255, 0.25);
    user-select: none;
  }
  :global(.mermaid-container svg) {
    max-width: 100%;
    height: auto;
  }
  :global(.problem-ref) {
    display: inline-flex;
    align-items: center;
    border: 1px solid rgb(var(--c-bg-border));
    border-radius: 0.375rem;
    background: rgb(var(--c-bg-card));
    padding: 0 0.35rem;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 0.9em;
    color: rgb(var(--c-accent)) !important;
    text-decoration: none !important;
  }
  :global(.problem-ref:hover) {
    border-color: rgb(var(--c-accent) / 0.5);
    background: rgb(var(--c-accent) / 0.12);
  }
</style>
