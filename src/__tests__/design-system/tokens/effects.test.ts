import { describe, it, expect } from 'vitest';
import { effects } from '@/design-system/tokens/effects';

describe('Effect Tokens', () => {
  it('should have all 4 effect types', () => {
    expect(effects).toHaveProperty('shimmer');
    expect(effects).toHaveProperty('glow');
    expect(effects).toHaveProperty('pulse');
    expect(effects).toHaveProperty('glass');
  });

  it('should have shimmer with correct duration', () => {
    expect(effects.shimmer.duration).toBe('2.5s');
    expect(effects.shimmer.type).toBe('animation');
  });

  it('should have glow with shadow type', () => {
    expect(effects.glow.type).toBe('shadow');
    expect(effects.glow.properties).toHaveProperty('boxShadow');
  });

  it('should have pulse with 2s duration', () => {
    expect(effects.pulse.duration).toBe('2s');
    expect(effects.pulse.type).toBe('animation');
  });

  it('should have glass effect with 24px blur', () => {
    expect(effects.glass.type).toBe('filter');
    expect(effects.glass.properties.backdropFilter).toContain('blur(24px)');
  });

  it('should have valid timing functions', () => {
    const validTimings = ['linear', 'ease', 'ease-in', 'ease-out', 'ease-in-out', 'cubic-bezier', 'none'];
    Object.values(effects).forEach(effect => {
      const isValid = validTimings.some(timing => effect.timingFunction.includes(timing));
      expect(isValid).toBe(true);
    });
  });

  it('should have usage documentation for each effect', () => {
    Object.values(effects).forEach(effect => {
      expect(effect.usage).toBeDefined();
      expect(Array.isArray(effect.usage)).toBe(true);
      expect(effect.usage.length).toBeGreaterThan(0);
    });
  });
});
