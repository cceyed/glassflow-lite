import { describe, it, expect } from 'vitest';
import { colors } from '@/design-system/tokens/colors';

describe('Color Tokens', () => {
  it('should have pure black background', () => {
    expect(colors.background).toBe('#000000');
  });

  it('should have white primary text', () => {
    expect(colors.textPrimary).toBe('#ffffff');
  });

  it('should have elevated surface color', () => {
    expect(colors.surface).toBe('#0f0f0f');
  });

  it('should have exactly 4 glass blue variants', () => {
    const glassBlueKeys = Object.keys(colors).filter(key => key.startsWith('glass'));
    expect(glassBlueKeys).toHaveLength(4);
    expect(glassBlueKeys).toContain('glassBase');
    expect(glassBlueKeys).toContain('glassBorder');
    expect(glassBlueKeys).toContain('glassGlow');
    expect(glassBlueKeys).toContain('glassBright');
  });

  it('should have valid hex colors', () => {
    const hexRegex = /^#[0-9A-Fa-f]{6}$/;
    const rgbaRegex = /^rgba\(/;
    Object.entries(colors).forEach(([key, value]) => {
      // Skip glass colors and effect colors (they use rgba)
      if (!key.startsWith('glass') && !['shimmer', 'glowWhite', 'selection', 'overlay'].includes(key)) {
        expect(value).toMatch(hexRegex);
      } else if (key.startsWith('glass') || ['shimmer', 'glowWhite', 'selection', 'overlay'].includes(key)) {
        expect(value).toMatch(rgbaRegex);
      }
    });
  });

  it('should have border colors for subtle and dividers', () => {
    expect(colors.border).toBe('#1a1a1a');
    expect(colors.divider).toBe('#2a2a2a');
  });

  it('should have secondary and tertiary text colors', () => {
    expect(colors.textSecondary).toBeDefined();
    expect(colors.textTertiary).toBeDefined();
    expect(colors.textDisabled).toBeDefined();
  });

  it('should have glass blue colors with rgba format', () => {
    expect(colors.glassBase).toContain('rgba');
    expect(colors.glassBorder).toContain('rgba');
    expect(colors.glassGlow).toContain('rgba');
    expect(colors.glassBright).toContain('rgba');
  });
});
