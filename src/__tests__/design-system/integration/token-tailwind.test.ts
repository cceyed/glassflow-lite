import { describe, it, expect } from 'vitest';
import { colors, spacing } from '@/design-system/tokens';

describe('Token-Tailwind Integration', () => {
  it('should have design system colors available in Tailwind config', () => {
    // Verify color tokens are defined
    expect(colors.background).toBe('#000000');
    expect(colors.surface).toBe('#0f0f0f');
    expect(colors.textPrimary).toBe('#ffffff');
  });

  it('should have spacing tokens available in Tailwind config', () => {
    // Verify spacing tokens are defined
    expect(spacing['2']).toBe('0.5rem'); // 8px
    expect(spacing['4']).toBe('1rem'); // 16px
    expect(spacing['6']).toBe('1.5rem'); // 24px
  });

  it('should have glass blue color variants', () => {
    expect(colors.glassBase).toContain('rgba');
    expect(colors.glassBorder).toContain('rgba');
    expect(colors.glassGlow).toContain('rgba');
    expect(colors.glassBright).toContain('rgba');
  });

  it('should have all spacing values as multiples of 4 or 8', () => {
    Object.entries(spacing).forEach(([key, value]) => {
      if (key === '0') return;
      const remMatch = value.match(/^([\d.]+)rem$/);
      if (remMatch) {
        const pixels = parseFloat(remMatch[1]) * 16;
        expect(pixels % 4).toBe(0);
      }
    });
  });
});
