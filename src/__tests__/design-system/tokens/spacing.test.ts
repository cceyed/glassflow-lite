import { describe, it, expect } from 'vitest';
import { spacing } from '@/design-system/tokens/spacing';

describe('Spacing Tokens', () => {
  it('should have spacing values as multiples of 8px or 4px', () => {
    Object.entries(spacing).forEach(([key, value]) => {
      // Convert rem to pixels (1rem = 16px)
      const remMatch = value.match(/^([\d.]+)rem$/);
      if (remMatch) {
        const pixels = parseFloat(remMatch[1]) * 16;
        // Should be multiple of 4 (allowing half-steps) or 8
        expect(pixels % 4).toBe(0);
      }
    });
  });

  it('should have common spacing values', () => {
    // Common 8px multiples
    const hasEightPx = Object.values(spacing).some(v => v === '0.5rem'); // 8px
    const hasSixteenPx = Object.values(spacing).some(v => v === '1rem'); // 16px
    const hasTwentyFourPx = Object.values(spacing).some(v => v === '1.5rem'); // 24px
    const hasThirtyTwoPx = Object.values(spacing).some(v => v === '2rem'); // 32px

    expect(hasEightPx).toBe(true);
    expect(hasSixteenPx).toBe(true);
    expect(hasTwentyFourPx).toBe(true);
    expect(hasThirtyTwoPx).toBe(true);
  });

  it('should use rem units', () => {
    Object.values(spacing).forEach(value => {
      // '0' is valid without units
      if (value !== '0') {
        expect(value).toMatch(/^\d+(\.\d+)?rem$/);
      }
    });
  });

  it('should have sequential keys', () => {
    const keys = Object.keys(spacing);
    expect(keys.length).toBeGreaterThan(0);
    // Keys should be numeric strings
    keys.forEach(key => {
      expect(/^\d+$/.test(key)).toBe(true);
    });
  });
});
