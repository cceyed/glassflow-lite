/**
 * Design Token Contracts
 * Type definitions for all design tokens in the system
 */

// ============================================================================
// Color Tokens
// ============================================================================

export interface ColorToken {
  name: string;
  hex: string;
  rgb: { r: number; g: number; b: number };
  rgba: string;
  usage: string[];
  category: "base" | "text" | "border" | "accent" | "effect";
}

export interface ColorPalette {
  // Base colors
  background: string;
  surface: string;
  border: string;
  divider: string;

  // Text colors
  textPrimary: string;
  textSecondary: string;
  textTertiary: string;
  textDisabled: string;

  // Glass accent (branding only)
  glassBase: string;
  glassBorder: string;
  glassGlow: string;
  glassBright: string;

  // Effect colors
  shimmer: string;
  glowWhite: string;
  selection: string;
  overlay: string;
}

// ============================================================================
// Typography Tokens
// ============================================================================

export interface TypographyToken {
  name: string;
  fontSize: string;
  fontWeight: 300 | 400 | 500 | 600 | 700;
  lineHeight: number;
  fontFamily: "ui" | "mono";
  letterSpacing?: string;
}

export interface TypeScale {
  hero: TypographyToken;
  title: TypographyToken;
  heading: TypographyToken;
  large: TypographyToken;
  body: TypographyToken;
  small: TypographyToken;
  caption: TypographyToken;
  tiny: TypographyToken;
}

export interface FontFamily {
  ui: string;
  mono: string;
}

// ============================================================================
// Effect Tokens
// ============================================================================

export interface EffectToken {
  name: string;
  type: "animation" | "shadow" | "filter";
  properties: Record<string, string>;
  duration: string;
  timingFunction: string;
  usage: string[];
}

export interface EffectLibrary {
  shimmer: EffectToken;
  glow: EffectToken;
  pulse: EffectToken;
  glass: EffectToken;
}

// ============================================================================
// Spacing Tokens
// ============================================================================

export interface SpacingToken {
  name: string;
  value: string;
  pixels: number;
  multiple: number;
}

export interface SpacingScale {
  [key: string]: string; // e.g., "1": "0.5rem", "2": "1rem"
}

// ============================================================================
// Token Validation
// ============================================================================

export interface TokenValidation {
  isValidHex(hex: string): boolean;
  isValidRGB(rgb: { r: number; g: number; b: number }): boolean;
  isMultipleOf8(pixels: number): boolean;
  isValidFontWeight(weight: number): boolean;
  isValidDuration(duration: string): boolean;
}

// ============================================================================
// Token Exports
// ============================================================================

export interface DesignTokens {
  colors: ColorPalette;
  typography: {
    scale: TypeScale;
    fonts: FontFamily;
  };
  effects: EffectLibrary;
  spacing: SpacingScale;
}
