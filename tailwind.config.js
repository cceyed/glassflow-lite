/** @type {import('tailwindcss').Config} */
import { colors } from './src/design-system/tokens/colors.ts';
import { spacing } from './src/design-system/tokens/spacing.ts';

export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Design system colors
        background: colors.background,
        surface: colors.surface,
        border: colors.border,
        divider: colors.divider,
        'text-primary': colors.textPrimary,
        'text-secondary': colors.textSecondary,
        'text-tertiary': colors.textTertiary,
        'text-disabled': colors.textDisabled,
        // Glass blue variants
        'glass-blue': {
          50: '#e6f0ff',
          100: '#cce0ff',
          200: '#99c2ff',
          300: '#66a3ff',
          400: '#3385ff',
          500: '#0066ff',
          600: '#0052cc',
          700: '#003d99',
          800: '#002966',
          900: '#001433',
        },
      },
      spacing,
      fontSize: {
        'hero': '48px',
        'title': '32px',
        'heading': '24px',
        'large': '18px',
        'body': '16px',
        'small': '14px',
        'caption': '12px',
        'tiny': '10px',
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'shimmer': 'shimmer 2.5s linear infinite',
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        shimmer: {
          '0%': { backgroundPosition: '-1000px 0' },
          '100%': { backgroundPosition: '1000px 0' },
        },
        glow: {
          '0%': { boxShadow: '0 0 5px rgba(0, 102, 255, 0.5), 0 0 10px rgba(0, 102, 255, 0.3)' },
          '100%': { boxShadow: '0 0 20px rgba(0, 102, 255, 0.8), 0 0 30px rgba(0, 102, 255, 0.5)' },
        },
      },
      backdropBlur: {
        'glass': '10px',
      },
    },
  },
  plugins: [
    function({ addUtilities }) {
      const newUtilities = {
        '.glass-effect': {
          background: 'rgba(255, 255, 255, 0.05)',
          backdropFilter: 'blur(10px)',
          '-webkit-backdrop-filter': 'blur(10px)',
          border: '1px solid rgba(255, 255, 255, 0.1)',
          borderRadius: '0.5rem',
        },
        '.glass-blue-glow': {
          boxShadow: '0 0 20px rgba(0, 102, 255, 0.3), 0 0 40px rgba(0, 102, 255, 0.2), inset 0 0 20px rgba(0, 102, 255, 0.1)',
        },
        '.white-glow': {
          boxShadow: '0 0 20px rgba(255, 255, 255, 0.3), 0 0 40px rgba(255, 255, 255, 0.2)',
        },
        '.text-shimmer': {
          background: 'linear-gradient(90deg, rgba(255, 255, 255, 0.8) 0%, rgba(255, 255, 255, 1) 50%, rgba(255, 255, 255, 0.8) 100%)',
          backgroundSize: '200% auto',
          backgroundClip: 'text',
          '-webkit-background-clip': 'text',
          '-webkit-text-fill-color': 'transparent',
          animation: 'shimmer 3s linear infinite',
        },
      }
      addUtilities(newUtilities)
    }
  ],
}
