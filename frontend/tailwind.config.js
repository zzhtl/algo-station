/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: ['./src/**/*.{html,js,svelte,ts}', '../content/articles/**/*.md'],
  theme: {
    extend: {
      colors: {
        // 调色板由 app.css 的 CSS 变量驱动（:root 亮色 / .dark 暗色），
        // 模板里的 bg-bg-card / text-ink 等类名无需改动即可随主题切换。
        bg: {
          DEFAULT: 'rgb(var(--c-bg) / <alpha-value>)',
          soft: 'rgb(var(--c-bg-soft) / <alpha-value>)',
          card: 'rgb(var(--c-bg-card) / <alpha-value>)',
          border: 'rgb(var(--c-bg-border) / <alpha-value>)'
        },
        accent: {
          DEFAULT: 'rgb(var(--c-accent) / <alpha-value>)',
          hover: 'rgb(var(--c-accent-hover) / <alpha-value>)',
          muted: 'rgb(var(--c-accent-muted) / <alpha-value>)'
        },
        easy: 'rgb(var(--c-easy) / <alpha-value>)',
        medium: 'rgb(var(--c-medium) / <alpha-value>)',
        hard: 'rgb(var(--c-hard) / <alpha-value>)',
        ink: {
          DEFAULT: 'rgb(var(--c-ink) / <alpha-value>)',
          mute: 'rgb(var(--c-ink-mute) / <alpha-value>)',
          dim: 'rgb(var(--c-ink-dim) / <alpha-value>)'
        }
      },
      fontFamily: {
        sans: ['ui-sans-serif', 'system-ui', '-apple-system', 'PingFang SC', 'Microsoft YaHei', 'sans-serif'],
        mono: ['ui-monospace', 'SFMono-Regular', 'JetBrains Mono', 'Menlo', 'monospace']
      }
    }
  },
  plugins: [require('@tailwindcss/typography')]
};
