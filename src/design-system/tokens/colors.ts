/**
 * Color Tokens
 * Monochrome palette with strategic glass blue accents
 */

export const colors = {
  // Base colors
  background: '#000000',
  surface: '#0f0f0f',
  border: '#1a1a1a',
  divider: '#2a2a2a',

  // Text colors
  textPrimary: '#ffffff',
  textSecondary: '#b0b0b0',
  textTertiary: '#707070',
  textDisabled: '#404040',

  // Glass accent (branding ONLY)
  glassBase: 'rgba(100, 150, 255, 0.08)',
  glassBorder: 'rgba(100, 150, 255, 0.25)',
  glassGlow: 'rgba(100, 150, 255, 0.4)',
  glassBright: 'rgba(100, 150, 255, 0.6)',

  // Effect colors
  shimmer: 'rgba(255, 255, 255, 0.6)',
  glowWhite: 'rgba(255, 255, 255, 0.3)',
  selection: 'rgba(255, 255, 255, 0.12)',
  overlay: 'rgba(0, 0, 0, 0.8)',
} as const;

export type ColorKey = keyof typeof colors;
