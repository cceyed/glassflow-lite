# Data Model: Foundation & Visual System

**Feature**: 001-foundation-visual-system  
**Date**: 2025-09-30  
**Status**: Complete

## Design Token Entities

### ColorToken
Represents a named color value in the design system.

**Attributes**:
- `name`: string - Token identifier (e.g., "background", "surface", "glass-blue-500")
- `hex`: string - Hexadecimal color value (e.g., "#000000")
- `rgb`: {r: number, g: number, b: number} - RGB values
- `rgba`: string - RGBA string for opacity variants (e.g., "rgba(0, 102, 255, 0.4)")
- `usage`: string[] - Contexts where this color should be used
- `category`: "base" | "text" | "border" | "accent" | "effect"

**Validation Rules**:
- `hex` must be valid 6-digit hex color
- `rgb` values must be 0-255
- `usage` must not be empty
- Glass blue colors must have `category: "accent"`

**Relationships**:
- Used by Component styles
- Referenced in Tailwind config

---

### TypographyToken
Represents a text style in the type scale.

**Attributes**:
- `name`: string - Scale level (e.g., "hero", "title", "body")
- `fontSize`: string - Size in px or rem (e.g., "48px", "3rem")
- `fontWeight`: 300 | 400 | 500 | 600 | 700
- `lineHeight`: number - Unitless line height multiplier
- `fontFamily`: "ui" | "mono" - Font stack to use
- `letterSpacing`: string - Optional letter spacing

**Validation Rules**:
- `fontSize` must be positive number
- `fontWeight` must be one of allowed values
- `lineHeight` typically 1.2-1.6 for readability

**Relationships**:
- Applied to Component text elements
- Extended in Tailwind config

---

### EffectToken
Represents a visual effect (animation, shadow, blur).

**Attributes**:
- `name`: string - Effect identifier (e.g., "shimmer", "glow", "pulse")
- `type`: "animation" | "shadow" | "filter"
- `properties`: Record<string, string> - CSS properties
- `duration`: string - Animation duration (e.g., "2s", "2.5s")
- `timingFunction`: string - Easing function
- `usage`: string[] - When to apply this effect

**Validation Rules**:
- `duration` must be valid CSS time value
- `timingFunction` must be valid CSS easing
- Animation effects must have keyframes defined

**Relationships**:
- Applied to Components based on state
- Defined in animations.css

---

### SpacingToken
Represents a spacing value in the 8px grid system.

**Attributes**:
- `name`: string - Spacing identifier (e.g., "xs", "sm", "md")
- `value`: string - Size in rem (e.g., "0.5rem", "1rem")
- `pixels`: number - Equivalent pixel value
- `multiple`: number - Multiple of 8px base unit

**Validation Rules**:
- `pixels` must be multiple of 8 (or 4 for half-steps)
- `value` must match pixels (1rem = 16px)
- `multiple` must be positive number

**Relationships**:
- Used for padding, margin, gaps in Components
- Extended in Tailwind spacing scale

---

## Component Entities

### Component (Base)
Abstract representation of a UI component.

**Attributes**:
- `name`: string - Component identifier
- `props`: ComponentProps - Type-safe prop interface
- `states`: ComponentState[] - Supported interactive states
- `dimensions`: {width?: string, height?: string, minWidth?: string, minHeight?: string}
- `tokens`: {colors: ColorToken[], typography: TypographyToken[], effects: EffectToken[], spacing: SpacingToken[]}

**States**:
- `default`: Base appearance
- `hover`: Mouse over
- `active`: Mouse down / selected
- `focus`: Keyboard focus
- `disabled`: Non-interactive

**Relationships**:
- Uses ColorTokens for styling
- Uses TypographyTokens for text
- Uses EffectTokens for animations
- Uses SpacingTokens for layout

---

### TitleBar (Component)
Top bar with logo, project name, and window controls.

**Specific Attributes**:
- `height`: "40px" (fixed)
- `sections`: {left: "logo", center: "project-name", right: "window-controls"}
- `background`: ColorToken("background")
- `borderBottom`: ColorToken("border")

**Props**:
- `projectName`: string
- `onMinimize`: () => void
- `onMaximize`: () => void
- `onClose`: () => void

---

### CommandInput (Component)
Command input field with focus effects.

**Specific Attributes**:
- `height`: "60px" (fixed)
- `fontSize`: TypographyToken("large")
- `placeholder`: string
- `effects`: {focus: ["shimmer", "glow"]}

**Props**:
- `value`: string
- `onChange`: (value: string) => void
- `onSubmit`: (value: string) => void
- `placeholder`: string

**State Transitions**:
- `default` → `focus`: Apply shimmer + blue glow
- `focus` → `default`: Remove effects

---

### AgentStatusBar (Component)
Bar showing 4 agent indicators with expandable reasoning.

**Specific Attributes**:
- `height`: "48px" (fixed)
- `agentCount`: 4
- `background`: ColorToken("surface")
- `effects`: {hover: ["expand-panel"]}

**Props**:
- `agents`: Array<{name: string, status: "idle" | "active" | "complete", progress: number}>
- `onAgentClick`: (agentId: string) => void

**State Transitions**:
- `default` → `hover`: Expand reasoning panel
- `hover` → `default`: Collapse panel

---

### StatusBar (Component)
Bottom bar with project info and status.

**Specific Attributes**:
- `height`: "32px" (fixed)
- `sections`: {left: "project-path", center: "phase", right: "time-elapsed"}
- `background`: ColorToken("background")
- `borderTop`: ColorToken("border")

**Props**:
- `projectPath`: string
- `currentPhase`: string
- `timeElapsed`: string

---

### Button (Component)
Interactive button with variants and states.

**Specific Attributes**:
- `variants`: ["primary", "secondary", "ghost"]
- `effects`: {hover: ["shimmer"], active: ["scale-down"]}

**Props**:
- `variant`: "primary" | "secondary" | "ghost"
- `disabled`: boolean
- `onClick`: () => void
- `children`: ReactNode

**State Transitions**:
- `default` → `hover`: Apply shimmer effect
- `hover` → `active`: Scale down slightly
- Any → `disabled`: Reduce opacity, disable interactions

---

### Card (Component)
Container with elevated background and borders.

**Specific Attributes**:
- `background`: ColorToken("surface")
- `border`: ColorToken("border")
- `borderRadius`: "8px"
- `padding`: SpacingToken("6") // 24px
- `effects`: {hover: ["brighten-border"], active: ["glow"]}

**Props**:
- `children`: ReactNode
- `onClick`: () => void (optional)

**State Transitions**:
- `default` → `hover`: Brighten border
- `hover` → `active`: Add glow effect

---

### Modal (Component)
Overlay dialog with backdrop blur.

**Specific Attributes**:
- `overlay`: {background: "rgba(0, 0, 0, 0.9)", backdropBlur: "10px"}
- `effects`: {enter: ["scale-up"], exit: ["scale-down"]}

**Props**:
- `isOpen`: boolean
- `onClose`: () => void
- `children`: ReactNode

**State Transitions**:
- `closed` → `open`: Scale up animation
- `open` → `closed`: Scale down animation

---

## Layout Entities

### LayoutRegion
Represents a section of the main application layout.

**Attributes**:
- `name`: string - Region identifier
- `height`: string - Fixed or "flexible"
- `component`: Component - Component that fills this region
- `order`: number - Vertical stacking order

**Regions**:
1. TitleBar (40px, fixed)
2. CommandInput (60px, fixed)
3. ContentArea (flexible)
4. AgentStatusBar (48px, fixed)
5. StatusBar (32px, fixed)

**Validation Rules**:
- Total fixed height must not exceed viewport
- Flexible regions must have min-height
- Order must be sequential

---

## Type Definitions

### ComponentProps (Base Interface)
```typescript
interface ComponentProps {
  className?: string;
  style?: React.CSSProperties;
  testId?: string;
}
```

### ComponentState
```typescript
type ComponentState = 
  | "default"
  | "hover"
  | "active"
  | "focus"
  | "disabled";
```

### TokenCategory
```typescript
type TokenCategory = 
  | "base"
  | "text"
  | "border"
  | "accent"
  | "effect";
```

---

## Relationships Summary

```
ColorToken ──> Component (styling)
           └─> Tailwind Config

TypographyToken ──> Component (text)
                └─> Tailwind Config

EffectToken ──> Component (animations)
            └─> animations.css

SpacingToken ──> Component (layout)
             └─> Tailwind Config

Component ──> LayoutRegion (placement)
          └─> ComponentState (behavior)

LayoutRegion ──> MainLayout (composition)
```

---

## Validation Summary

All entities have:
- Clear attribute definitions
- Validation rules
- Relationship mappings
- State transitions (where applicable)

Ready to generate contracts and tests in Phase 1.
