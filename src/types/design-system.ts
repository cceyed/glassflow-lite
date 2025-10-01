/**
 * Base TypeScript types for the design system
 * Derived from contracts in specs/001-foundation-visual-system/contracts/
 */

import { ReactNode, CSSProperties } from 'react';

// ============================================================================
// Base Component Props
// ============================================================================

export interface BaseComponentProps {
  className?: string;
  style?: CSSProperties;
  testId?: string;
}

export type ComponentState = 'default' | 'hover' | 'active' | 'focus' | 'disabled';

// ============================================================================
// Token Types
// ============================================================================

export interface ColorToken {
  name: string;
  hex: string;
  rgb: { r: number; g: number; b: number };
  rgba: string;
  usage: string[];
  category: 'base' | 'text' | 'border' | 'accent' | 'effect';
}

export interface TypographyToken {
  name: string;
  fontSize: string;
  fontWeight: 300 | 400 | 500 | 600 | 700;
  lineHeight: number;
  fontFamily: 'ui' | 'mono';
  letterSpacing?: string;
}

export interface EffectToken {
  name: string;
  type: 'animation' | 'shadow' | 'filter';
  properties: Record<string, string>;
  duration: string;
  timingFunction: string;
  usage: string[];
}

export interface SpacingToken {
  name: string;
  value: string;
  pixels: number;
  multiple: number;
}

// ============================================================================
// Component Prop Types
// ============================================================================

export type ButtonVariant = 'primary' | 'secondary' | 'ghost';

export interface AgentStatus {
  id: string;
  name: string;
  status: 'idle' | 'active' | 'complete';
  progress: number;
  reasoning?: string[];
}

export interface LayoutRegion {
  name: string;
  height: string;
  component: string;
  order: number;
}

export interface WindowConfig {
  defaultWidth: 1400;
  defaultHeight: 900;
  minWidth: 1200;
  minHeight: 750;
}
