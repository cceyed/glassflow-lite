/**
 * Effect Tokens
 * Animations and visual effects
 */

import { EffectToken } from '@/types/design-system';

export const effects = {
  shimmer: {
    name: 'shimmer',
    type: 'animation',
    properties: {
      backgroundImage: 'linear-gradient(90deg, rgba(255,255,255,0.8) 0%, rgba(255,255,255,1) 50%, rgba(255,255,255,0.8) 100%)',
      backgroundSize: '200% auto',
    },
    duration: '2.5s',
    timingFunction: 'linear',
    usage: ['loading states', 'brand elements', 'hover effects'],
  } as EffectToken,
  glow: {
    name: 'glow',
    type: 'shadow',
    properties: {
      boxShadow: '0 0 20px rgba(0, 102, 255, 0.3), 0 0 40px rgba(0, 102, 255, 0.2), inset 0 0 20px rgba(0, 102, 255, 0.1)',
    },
    duration: '2s',
    timingFunction: 'ease-in-out',
    usage: ['active elements', 'focus states'],
  } as EffectToken,
  pulse: {
    name: 'pulse',
    type: 'animation',
    properties: {
      scale: '1.0 → 1.02 → 1.0',
      opacity: '1.0 → 0.85 → 1.0',
    },
    duration: '2s',
    timingFunction: 'ease-in-out',
    usage: ['active agents', 'processing states'],
  } as EffectToken,
  glass: {
    name: 'glass',
    type: 'filter',
    properties: {
      backdropFilter: 'blur(24px)',
      background: 'rgba(100, 150, 255, 0.08)',
      border: '1px solid rgba(100, 150, 255, 0.25)',
    },
    duration: '0s',
    timingFunction: 'none',
    usage: ['logo', 'brand accents ONLY'],
  } as EffectToken,
} as const;

export type EffectKey = keyof typeof effects;
