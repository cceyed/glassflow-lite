/**
 * Typography Tokens
 * 8-level type scale with system fonts
 */

import { TypographyToken } from '@/types/design-system';

export const typography = {
  scale: {
    hero: {
      name: 'hero',
      fontSize: '48px',
      fontWeight: 700,
      lineHeight: 1.2,
      fontFamily: 'ui',
    } as TypographyToken,
    title: {
      name: 'title',
      fontSize: '32px',
      fontWeight: 600,
      lineHeight: 1.25,
      fontFamily: 'ui',
    } as TypographyToken,
    heading: {
      name: 'heading',
      fontSize: '24px',
      fontWeight: 600,
      lineHeight: 1.3,
      fontFamily: 'ui',
    } as TypographyToken,
    large: {
      name: 'large',
      fontSize: '18px',
      fontWeight: 500,
      lineHeight: 1.4,
      fontFamily: 'ui',
    } as TypographyToken,
    body: {
      name: 'body',
      fontSize: '16px',
      fontWeight: 400,
      lineHeight: 1.5,
      fontFamily: 'ui',
    } as TypographyToken,
    small: {
      name: 'small',
      fontSize: '14px',
      fontWeight: 400,
      lineHeight: 1.5,
      fontFamily: 'ui',
    } as TypographyToken,
    caption: {
      name: 'caption',
      fontSize: '12px',
      fontWeight: 400,
      lineHeight: 1.4,
      fontFamily: 'ui',
    } as TypographyToken,
    tiny: {
      name: 'tiny',
      fontSize: '10px',
      fontWeight: 400,
      lineHeight: 1.4,
      fontFamily: 'ui',
    } as TypographyToken,
  },
  fonts: {
    ui: "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif",
    mono: "source-code-pro, Menlo, Monaco, Consolas, 'Courier New', monospace",
  },
} as const;

export type TypeScaleKey = keyof typeof typography.scale;
