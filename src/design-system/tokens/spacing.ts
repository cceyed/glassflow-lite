/**
 * Spacing Tokens
 * 8px grid system (using rem, where 1rem = 16px)
 */

export const spacing = {
  '0': '0',
  '1': '0.25rem',  // 4px (half-step)
  '2': '0.5rem',   // 8px
  '3': '0.75rem',  // 12px
  '4': '1rem',     // 16px (2x8px)
  '5': '1.25rem',  // 20px
  '6': '1.5rem',   // 24px (3x8px)
  '8': '2rem',     // 32px (4x8px)
  '10': '2.5rem',  // 40px (5x8px)
  '12': '3rem',    // 48px (6x8px)
  '16': '4rem',    // 64px (8x8px)
  '20': '5rem',    // 80px (10x8px)
  '24': '6rem',    // 96px (12x8px)
  '32': '8rem',    // 128px (16x8px)
} as const;

export type SpacingKey = keyof typeof spacing;
