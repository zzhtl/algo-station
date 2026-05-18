import { marked } from 'marked';
import { codeToHtml } from 'shiki';

const escapeHtml = (s: string) =>
  s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');

const renderer = new marked.Renderer();

const originalCode = renderer.code.bind(renderer);
renderer.code = function (token) {
  const { text, lang } = token;
  if (lang === 'mermaid') {
    const encoded = escapeHtml(text);
    return `<div class="mermaid-container"><div class="mermaid-source" data-mermaid="${encoded}"></div></div>`;
  }
  return originalCode(token);
};

marked.use({
  renderer,
  gfm: true,
  breaks: false,
  async: false
});

export async function renderMarkdown(md: string): Promise<string> {
  const rawHtml = marked.parse(prioritizeSolutionLanguages(md)) as string;
  return await highlightCodeBlocks(stripLeetCodeLinks(rawHtml));
}

function prioritizeSolutionLanguages(md: string): string {
  return prioritizeAdjacentCodeBlocks(prioritizeDoocsTabs(md));
}

function prioritizeDoocsTabs(md: string): string {
  return md.replace(/<!-- tabs:start -->([\s\S]*?)<!-- tabs:end -->/g, (match, body: string) => {
    const firstHeading = body.search(/\n####\s+/);
    if (firstHeading < 0) return match;

    const prefix = body.slice(0, firstHeading);
    const sections = body
      .slice(firstHeading)
      .split(/(?=\n####\s+)/)
      .filter(Boolean);

    if (!sections.some((section) => languagePriority(sectionLabel(section)) < 2)) return match;

    const sorted = sections
      .map((section, index) => ({ section, index, priority: languagePriority(sectionLabel(section)) }))
      .sort((a, b) => a.priority - b.priority || a.index - b.index)
      .map(({ section }) => section)
      .join('');

    return `<!-- tabs:start -->${prefix}${sorted}<!-- tabs:end -->`;
  });
}

function prioritizeAdjacentCodeBlocks(md: string): string {
  return md.replace(/(?:```[^\n]*\n[\s\S]*?\n```\s*){2,}/g, (block) => {
    const fences = block.match(/```[^\n]*\n[\s\S]*?\n```\s*/g);
    if (!fences || !fences.some((fence) => languagePriority(fenceLang(fence)) < 2)) {
      return block;
    }

    return fences
      .map((fence, index) => ({ fence, index, priority: languagePriority(fenceLang(fence)) }))
      .sort((a, b) => a.priority - b.priority || a.index - b.index)
      .map(({ fence }) => fence)
      .join('');
  });
}

function sectionLabel(section: string): string {
  return section.match(/\n####\s+([^\n]+)/)?.[1] ?? '';
}

function fenceLang(fence: string): string {
  return fence.match(/^```([^\n]*)/)?.[1] ?? '';
}

function languagePriority(label: string): number {
  const normalized = label.trim().toLowerCase();
  if (normalized === 'go' || normalized === 'golang') return 0;
  if (normalized === 'rust' || normalized === 'rs') return 1;
  return 2;
}

function stripLeetCodeLinks(html: string): string {
  return html.replace(
    /<a\s+[^>]*href=(["'])https?:\/\/(?:leetcode\.cn|leetcode\.com)\/[^"']*\1[^>]*>([\s\S]*?)<\/a>/gi,
    '$2'
  );
}

async function highlightCodeBlocks(html: string): Promise<string> {
  const re = /<pre><code(?: class="language-([^"]+)")?>([\s\S]*?)<\/code><\/pre>/g;
  const matches: { match: string; lang: string; code: string }[] = [];
  let m: RegExpExecArray | null;
  while ((m = re.exec(html)) !== null) {
    matches.push({ match: m[0], lang: (m[1] || 'text').toLowerCase(), code: decodeEntities(m[2]) });
  }
  if (matches.length === 0) return html;

  const replacements = await Promise.all(
    matches.map(async ({ match, lang, code }) => {
      try {
        const highlighted = await codeToHtml(code, {
          lang: mapLang(lang),
          theme: 'github-dark-dimmed'
        });
        return { match, html: wrapCode(highlighted, lang) };
      } catch {
        const safe = escapeHtml(code);
        return {
          match,
          html: `<pre><code class="language-${escapeHtml(lang)}">${safe}</code></pre>`
        };
      }
    })
  );

  let out = html;
  for (const r of replacements) {
    out = out.replace(r.match, () => r.html);
  }
  return out;
}

function wrapCode(highlighted: string, lang: string): string {
  return `<div class="code-block" data-lang="${escapeHtml(lang)}">${highlighted}</div>`;
}

function decodeEntities(s: string): string {
  return s
    .replace(/&amp;/g, '&')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'");
}

function mapLang(lang: string): string {
  const map: Record<string, string> = {
    cpp: 'cpp',
    'c++': 'cpp',
    js: 'javascript',
    ts: 'typescript',
    py: 'python',
    rs: 'rust',
    sh: 'bash',
    shell: 'bash'
  };
  return map[lang] ?? lang;
}
