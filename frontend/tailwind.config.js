/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: ['./src/**/*.{html,js,svelte,ts}', '../content/articles/**/*.md'],
  theme: {
    extend: {
      colors: {
        bg: {
          DEFAULT: '#0b0d12',
          soft: '#11141b',
          card: '#161a23',
          border: '#252b39'
        },
        accent: {
          DEFAULT: '#7c9cff',
          hover: '#94aeff',
          muted: '#3d4a7a'
        },
        easy: '#22c55e',
        medium: '#f59e0b',
        hard: '#ef4444',
        ink: {
          DEFAULT: '#e6e8ee',
          mute: '#a0a6b3',
          dim: '#6b7280'
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
