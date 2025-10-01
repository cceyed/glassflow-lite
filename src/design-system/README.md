# Glassflow Design System

Complete design system with tokens, components, and styles following a monochrome aesthetic with strategic glass blue accents.

## Installation

The design system is already integrated into the project. Import components and tokens as needed:

```typescript
// Import components
import { Button, Card, Modal, TitleBar, CommandInput } from '@/design-system/components';

// Import tokens
import { colors, typography, effects, spacing } from '@/design-system/tokens';
```

## Design Tokens

### Colors

```typescript
import { colors } from '@/design-system/tokens';

// Monochrome palette
colors.background    // #000000 - Pure black
colors.surface       // #0f0f0f - Elevated surfaces
colors.textPrimary   // #ffffff - Primary text
colors.textSecondary // #b0b0b0 - Secondary text

// Glass blue (branding ONLY)
colors.glassBase     // rgba(100, 150, 255, 0.08)
colors.glassBorder   // rgba(100, 150, 255, 0.25)
colors.glassGlow     // rgba(100, 150, 255, 0.4)
colors.glassBright   // rgba(100, 150, 255, 0.6)
```

### Typography

```typescript
import { typography } from '@/design-system/tokens';

// 8-level type scale
typography.scale.hero    // 48px, 700 weight
typography.scale.title   // 32px, 600 weight
typography.scale.heading // 24px, 600 weight
typography.scale.large   // 18px, 500 weight
typography.scale.body    // 16px, 400 weight
typography.scale.small   // 14px, 400 weight
typography.scale.caption // 12px, 400 weight
typography.scale.tiny    // 10px, 400 weight
```

### Effects

```typescript
import { effects } from '@/design-system/tokens';

effects.shimmer // 2.5s diagonal gradient animation
effects.glow    // Multi-layer box-shadow
effects.pulse   // 2s scale + opacity animation
effects.glass   // 24px backdrop blur effect
```

### Spacing

8px grid system (all values are multiples of 8px or 4px):

```typescript
import { spacing } from '@/design-system/tokens';

spacing['2']  // 0.5rem (8px)
spacing['4']  // 1rem (16px)
spacing['6']  // 1.5rem (24px)
spacing['8']  // 2rem (32px)
```

## Components

### Button

```tsx
import { Button } from '@/design-system/components';

<Button variant="primary" onClick={() => {}}>
  Primary Button
</Button>

<Button variant="secondary">Secondary</Button>
<Button variant="ghost">Ghost</Button>
<Button disabled>Disabled</Button>
```

**Props:**
- `variant`: 'primary' | 'secondary' | 'ghost'
- `disabled`: boolean
- `onClick`: () => void
- `type`: 'button' | 'submit' | 'reset'

### Card

```tsx
import { Card } from '@/design-system/components';

<Card hoverable onClick={() => {}}>
  Card content
</Card>

<Card active>Active card with glow</Card>
```

**Props:**
- `hoverable`: boolean - Brighten border on hover
- `active`: boolean - Show blue glow
- `onClick`: () => void

### Modal

```tsx
import { Modal } from '@/design-system/components';

<Modal 
  isOpen={isOpen} 
  onClose={() => setIsOpen(false)}
  closeOnOverlayClick={true}
  closeOnEscape={true}
>
  Modal content
</Modal>
```

**Props:**
- `isOpen`: boolean
- `onClose`: () => void
- `closeOnOverlayClick`: boolean
- `closeOnEscape`: boolean

### CommandInput

```tsx
import { CommandInput } from '@/design-system/components';

<CommandInput
  value={value}
  onChange={setValue}
  onSubmit={handleSubmit}
  placeholder="Enter command..."
  autoFocus
/>
```

**Props:**
- `value`: string
- `onChange`: (value: string) => void
- `onSubmit`: (value: string) => void
- `placeholder`: string
- `autoFocus`: boolean

### TitleBar

```tsx
import { TitleBar } from '@/design-system/components';

<TitleBar
  projectName="My Project"
  onMinimize={() => {}}
  onMaximize={() => {}}
  onClose={() => {}}
/>
```

**Props:**
- `projectName`: string
- `onMinimize`: () => void
- `onMaximize`: () => void
- `onClose`: () => void

### AgentStatusBar

```tsx
import { AgentStatusBar } from '@/design-system/components';

<AgentStatusBar
  agents={[
    { id: '1', name: 'Planner', status: 'active', progress: 50 },
    { id: '2', name: 'Executor', status: 'idle', progress: 0 },
  ]}
  onAgentClick={(id) => console.log(id)}
/>
```

**Props:**
- `agents`: AgentStatus[]
- `onAgentClick`: (agentId: string) => void

### StatusBar

```tsx
import { StatusBar } from '@/design-system/components';

<StatusBar
  projectPath="/path/to/project"
  currentPhase="Implementation"
  timeElapsed="00:15:30"
/>
```

**Props:**
- `projectPath`: string
- `currentPhase`: string
- `timeElapsed`: string

## Custom Utilities

### Glass Effect

```tsx
<div className="glass-effect">
  Semi-transparent surface with blur
</div>
```

### Glass Blue Glow

```tsx
<div className="glass-blue-glow">
  Blue glow for active elements
</div>
```

### Text Shimmer

```tsx
<h1 className="text-shimmer">
  Animated shimmer text
</h1>
```

### Glass Brand (Logo/Branding ONLY)

```tsx
<div className="glass-brand">
  Glass effect with blue tint - USE SPARINGLY
</div>
```

## Design Principles

1. **Monochrome First**: Use black, white, and gray for 95% of the UI
2. **Glass Blue Sparingly**: Only for logo, brand elements, and key accents
3. **8px Grid**: All spacing must be multiples of 8px (or 4px for half-steps)
4. **Smooth Animations**: 60fps animations using Framer Motion
5. **Glass Morphism**: Blur and transparency for depth

## Testing

```bash
# Run unit tests
npm run test

# Run visual regression tests
npx playwright test

# Run specific test file
npm run test src/__tests__/design-system/tokens/colors.test.ts
```

## Contributing

When adding new components:

1. Create TypeScript interface in `src/types/design-system.ts`
2. Write tests first (TDD)
3. Implement component using design tokens
4. Export from `src/design-system/components/index.ts`
5. Update this README

## License

MIT
