import { describe, it, expect } from 'vitest';
import { typography } from '@/design-system/tokens/typography';

describe('Typography Tokens', () => {
  it('should have 8 type scale levels', () => {
    expect(typography.scale).toHaveProperty('hero');
    expect(typography.scale).toHaveProperty('title');
    expect(typography.scale).toHaveProperty('heading');
    expect(typography.scale).toHaveProperty('large');
    expect(typography.scale).toHaveProperty('body');
    expect(typography.scale).toHaveProperty('small');
    expect(typography.scale).toHaveProperty('caption');
    expect(typography.scale).toHaveProperty('tiny');
  });

  it('should have correct font sizes in descending order', () => {
    expect(typography.scale.hero.fontSize).toBe('48px');
    expect(typography.scale.title.fontSize).toBe('32px');
    expect(typography.scale.heading.fontSize).toBe('24px');
    expect(typography.scale.large.fontSize).toBe('18px');
    expect(typography.scale.body.fontSize).toBe('16px');
    expect(typography.scale.small.fontSize).toBe('14px');
    expect(typography.scale.caption.fontSize).toBe('12px');
    expect(typography.scale.tiny.fontSize).toBe('10px');
  });

  it('should have font weights between 300 and 700', () => {
    Object.values(typography.scale).forEach(token => {
      expect(token.fontWeight).toBeGreaterThanOrEqual(300);
      expect(token.fontWeight).toBeLessThanOrEqual(700);
    });
  });

  it('should have UI and mono font families', () => {
    expect(typography.fonts.ui).toBeDefined();
    expect(typography.fonts.mono).toBeDefined();
    expect(typography.fonts.ui).toContain('system');
    expect(typography.fonts.mono).toContain('mono');
  });

  it('should have line height defined for all scales', () => {
    Object.values(typography.scale).forEach(token => {
      expect(token.lineHeight).toBeGreaterThan(1);
      expect(token.lineHeight).toBeLessThan(2);
    });
  });

  it('should have correct font family types', () => {
    Object.values(typography.scale).forEach(token => {
      expect(['ui', 'mono']).toContain(token.fontFamily);
    });
  });
});
