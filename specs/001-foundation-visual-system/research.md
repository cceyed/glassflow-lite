# Research: Foundation & Visual System

**Feature**: 001-foundation-visual-system  
**Date**: 2025-09-30  
**Status**: Complete

## Research Questions

### 1. Glass Morphism Implementation in React
**Question**: What's the best approach to implement glass morphism effects with backdrop-filter in React/Tailwind?

**Decision**: Use Tailwind CSS custom utilities with backdrop-filter  
**Rationale**:
- Native CSS `backdrop-filter: blur()` provides best performance
- Tailwind utilities allow consistent application across components
- Framer Motion handles animations without conflicting with glass effects
- Browser support is excellent for desktop targets (Tauri)

**Alternatives Considered**:
- CSS-in-JS solutions (styled-components): More overhead, unnecessary for this use case
- Canvas-based blur: Performance issues, accessibility concerns
- SVG filters: Complex, harder to maintain

**Implementation Notes**:
- Use `backdrop-blur-xl` (24px) for glass effect
- Combine with `bg-opacity` for semi-transparent tint
- Add `border` with low opacity for definition

---

### 2. Animation Performance with Framer Motion
**Question**: How to ensure 60fps animations with glass effects and multiple components?

**Decision**: Use Framer Motion with GPU-accelerated properties  
**Rationale**:
- Framer Motion automatically uses `transform` and `opacity` (GPU-accelerated)
- `will-change` CSS hint for animated elements
- Avoid animating `filter` or `backdrop-filter` directly
- Use `layoutId` for shared element transitions

**Alternatives Considered**:
- Pure CSS animations: Less control, harder to coordinate
- GSAP: Additional dependency, overkill for this scope
- React Spring: Good but Framer Motion better integrated with React

**Implementation Notes**:
- Animate `scale`, `opacity`, `x`, `y` only
- Use `transition` prop for timing control
- Implement `AnimatePresence` for mount/unmount animations

---

### 3. Design Token Management
**Question**: How to structure design tokens for type safety and maintainability?

**Decision**: TypeScript const objects with as const assertions  
**Rationale**:
- Full type inference and autocomplete
- Single source of truth for tokens
- Easy to extend and maintain
- Can be imported anywhere in the app

**Alternatives Considered**:
- CSS custom properties only: No type safety
- JSON files: Requires build step, less type-safe
- Design token tools (Style Dictionary): Overkill for this scope

**Implementation Notes**:
```typescript
export const colors = {
  background: '#000000',
  surface: '#0f0f0f',
  // ...
} as const;

export type ColorToken = keyof typeof colors;
```

---

### 4. Component State Management
**Question**: How to handle component states (hover, active, focus, disabled)?

**Decision**: React hooks + Framer Motion variants  
**Rationale**:
- Framer Motion variants provide declarative state animations
- React hooks (`useState`, `useRef`) for internal state
- Props for external control
- Compound components for complex interactions

**Alternatives Considered**:
- Class-based components: Outdated, verbose
- Context for state: Unnecessary coupling
- External state library: Overkill for component-level state

**Implementation Notes**:
- Use `whileHover`, `whileTap`, `whileFocus` variants
- Combine with `useState` for toggle states
- Forward refs for DOM access

---

### 5. Testing Strategy
**Question**: How to test visual components and design tokens?

**Decision**: Vitest for unit tests, Playwright for visual regression  
**Rationale**:
- Vitest is fast and works well with Vite
- Playwright provides cross-browser visual testing
- Can snapshot test token values
- Component tests verify props and behavior

**Alternatives Considered**:
- Jest: Slower, requires more configuration with Vite
- Storybook: Good for documentation but not testing
- Chromatic: Paid service, unnecessary for this stage

**Implementation Notes**:
- Unit tests for token validation
- Component tests for prop handling
- Visual regression tests for appearance
- Integration tests for component interactions

---

### 6. Tailwind Configuration
**Question**: How to extend Tailwind with custom design tokens?

**Decision**: Extend theme in tailwind.config.js with token imports  
**Rationale**:
- Keeps tokens in TypeScript for type safety
- Tailwind config imports and spreads token objects
- Enables using tokens as Tailwind classes
- Single source of truth maintained

**Alternatives Considered**:
- Duplicate tokens in config: Maintenance nightmare
- CSS custom properties only: Loses Tailwind benefits
- Separate config file: Adds complexity

**Implementation Notes**:
```javascript
import { colors, spacing } from './src/design-system/tokens';

export default {
  theme: {
    extend: {
      colors,
      spacing,
    },
  },
};
```

---

### 7. 8px Grid System
**Question**: How to enforce 8px spacing multiples throughout the app?

**Decision**: Custom Tailwind spacing scale + ESLint rule  
**Rationale**:
- Tailwind spacing already uses 4px base (0.25rem)
- Extend with 8px multiples (2, 4, 6, 8, 10, 12, etc.)
- ESLint can warn on arbitrary values
- Design tokens enforce consistency

**Alternatives Considered**:
- CSS Grid with 8px gaps: Too rigid
- Manual enforcement: Error-prone
- PostCSS plugin: Adds build complexity

**Implementation Notes**:
- Use Tailwind's default spacing (already 4px based)
- Add custom values for larger 8px multiples
- Document spacing scale in design system

---

## Summary

All technical decisions are made and documented. The implementation will use:
- **Styling**: Tailwind CSS with custom utilities for glass effects
- **Animations**: Framer Motion with GPU-accelerated properties
- **Tokens**: TypeScript const objects with type inference
- **State**: React hooks + Framer Motion variants
- **Testing**: Vitest (unit) + Playwright (visual regression)
- **Grid**: Tailwind spacing scale (4px/8px multiples)

No blockers identified. Ready to proceed to Phase 1 (Design & Contracts).
