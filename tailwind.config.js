/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        void: {
          950: '#020408',
          900: '#050c14',
          800: '#091422',
          700: '#0d1e32',
          600: '#112840',
        },
        neon: {
          cyan:   '#00F5FF',
          violet: '#BF5AF2',
          amber:  '#FFB800',
          green:  '#00FF94',
          red:    '#FF3B5C',
        },
        data: {
          dim:    '#1a3a4a',
          mid:    '#2a6080',
          bright: '#4aadcc',
        },
      },
      fontFamily: {
        mono:  ['"JetBrains Mono"', '"Fira Code"', 'monospace'],
        sans:  ['"Outfit"', 'system-ui', 'sans-serif'],
      },
      animation: {
        'scanline':      'scanline 8s linear infinite',
        'pulse-border':  'pulse-border 2s ease-in-out infinite',
        'data-stream':   'data-stream 0.5s steps(1) infinite',
        'compile-burst': 'compile-burst 0.6s ease-out forwards',
        'fade-in':       'fade-in 0.3s ease-out forwards',
        'glow-pulse':    'glow-pulse 3s ease-in-out infinite',
        'slide-up':      'slide-up 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards',
      },
      keyframes: {
        scanline: {
          '0%':   { backgroundPosition: '0 0' },
          '100%': { backgroundPosition: '0 100vh' },
        },
        'pulse-border': {
          '0%, 100%': { borderColor: 'rgba(0,245,255,0.2)' },
          '50%':      { borderColor: 'rgba(0,245,255,0.7)' },
        },
        'data-stream': {
          '0%':  { opacity: '1' },
          '50%': { opacity: '0.4' },
        },
        'compile-burst': {
          '0%':   { transform: 'scale(1)',    opacity: '1' },
          '50%':  { transform: 'scale(1.05)', opacity: '0.8' },
          '100%': { transform: 'scale(1)',    opacity: '1' },
        },
        'fade-in': {
          from: { opacity: '0', transform: 'translateY(4px)' },
          to:   { opacity: '1', transform: 'translateY(0)' },
        },
        'glow-pulse': {
          '0%, 100%': { textShadow: '0 0 8px rgba(0,245,255,0.4)' },
          '50%':      { textShadow: '0 0 24px rgba(0,245,255,0.9), 0 0 48px rgba(0,245,255,0.4)' },
        },
        'slide-up': {
          from: { opacity: '0', transform: 'translateY(12px)' },
          to:   { opacity: '1', transform: 'translateY(0)' },
        },
      },
      boxShadow: {
        'neon-cyan':   '0 0 20px rgba(0,245,255,0.3), inset 0 0 20px rgba(0,245,255,0.05)',
        'neon-violet': '0 0 20px rgba(191,90,242,0.3), inset 0 0 20px rgba(191,90,242,0.05)',
        'neon-amber':  '0 0 20px rgba(255,184,0,0.3), inset 0 0 20px rgba(255,184,0,0.05)',
        'neon-red':    '0 0 20px rgba(255,59,92,0.4), inset 0 0 20px rgba(255,59,92,0.08)',
      },
    },
  },
  plugins: [],
}