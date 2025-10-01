# Quickstart: Foundation & Visual System

**Feature**: 001-foundation-visual-system  
**Purpose**: Validate that the design system is correctly implemented and functional  
**Estimated Time**: 10 minutes

## Prerequisites

- Node.js 18+ installed
- npm installed
- Project dependencies installed (`npm install`)
- Development server running (`npm run dev`)

## Test Scenarios

### Scenario 1: Design Tokens Are Accessible

**Objective**: Verify all design tokens are defined and importable

**Steps**:
1. Open `src/design-system/tokens/index.ts`
2. Verify exports for: `colors`, `typography`, `effects`, `spacing`
3. Import tokens in a test file:
   ```typescript
   import { colors, typography, effects, spacing } from '@/design-system/tokens';
   ```
4. Check that all token categories exist:
   - `colors.background === '#000000'`
   - `typography.scale.hero.fontSize === '48px'`
   - `effects.shimmer.duration === '2.5s'`
   - `spacing['2'] === '1rem'` (16px, 2x8px)

**Expected Result**: All tokens are defined with correct values

---

### Scenario 2: Color Palette Follows Monochrome + Glass Blue Rule

**Objective**: Verify glass blue is restricted to branding only

**Steps**:
1. Inspect `colors` token object
2. Count glass blue variants (should be exactly 4: base, border, glow, bright)
3. Verify all other colors are monochrome (black/white/gray)
4. Check usage documentation for glass blue tokens

**Expected Result**: 
- Glass blue colors: 4 variants
- All other colors: grayscale only
- Usage notes specify "branding only"

---

### Scenario 3: Typography Scale Has 8 Levels

**Objective**: Verify complete type scale from hero to tiny

**Steps**:
1. Inspect `typography.scale` object
2. Verify all 8 levels exist: hero, title, heading, large, body, small, caption, tiny
3. Check font sizes are in descending order
4. Verify font weights are within 300-700 range

**Expected Result**: 
- 8 typography levels defined
- Sizes: 48px → 10px
- Weights: 300, 400, 500, 600, 700

---

### Scenario 4: Effects Are Defined with Correct Properties

**Objective**: Verify all 4 effect types exist with proper configuration

**Steps**:
1. Inspect `effects` object
2. Verify shimmer effect:
   - Duration: 2.5s
   - Type: animation
   - Has keyframes defined
3. Verify glow effect:
   - Type: shadow
   - Has multi-layer box-shadow
4. Verify pulse effect:
   - Duration: 2s
   - Animates scale and opacity
5. Verify glass effect:
   - Has backdrop-filter: blur(24px)
   - Has semi-transparent background

**Expected Result**: All 4 effects properly configured

---

### Scenario 5: Spacing Uses 8px Grid

**Objective**: Verify all spacing values are multiples of 8px (or 4px for half-steps)

**Steps**:
1. Inspect `spacing` object
2. Convert all values to pixels
3. Verify each value is divisible by 4 or 8
4. Check common values exist: 8px, 16px, 24px, 32px, 48px

**Expected Result**: All spacing values follow 8px grid system

---

### Scenario 6: Components Are Importable

**Objective**: Verify all 7 components can be imported

**Steps**:
1. Import all components:
   ```typescript
   import {
     TitleBar,
     CommandInput,
     AgentStatusBar,
     StatusBar,
     Button,
     Card,
     Modal
   } from '@/design-system/components';
   ```
2. Verify each component is a valid React component
3. Check TypeScript types are available for props

**Expected Result**: All components import without errors

---

### Scenario 7: TitleBar Renders Correctly

**Objective**: Verify TitleBar component renders with correct layout

**Steps**:
1. Render TitleBar component:
   ```tsx
   <TitleBar 
     projectName="Test Project"
     onMinimize={() => {}}
     onMaximize={() => {}}
     onClose={() => {}}
   />
   ```
2. Verify height is 40px
3. Check logo is on the left
4. Check project name is centered
5. Check window controls are on the right
6. Verify background is pure black
7. Check bottom border exists

**Expected Result**: TitleBar renders with correct 40px height and layout

---

### Scenario 8: CommandInput Shows Focus Effects

**Objective**: Verify CommandInput displays shimmer and glow on focus

**Steps**:
1. Render CommandInput:
   ```tsx
   <CommandInput
     value=""
     onChange={() => {}}
     onSubmit={() => {}}
     placeholder="Enter command..."
   />
   ```
2. Click on the input to focus
3. Verify shimmer animation starts
4. Verify blue glow appears
5. Verify input height is 60px
6. Verify font size is 18px
7. Press Enter and verify onSubmit is called

**Expected Result**: Focus triggers shimmer + blue glow, Enter submits

---

### Scenario 9: Button Variants Render Correctly

**Objective**: Verify all 3 button variants have correct styling

**Steps**:
1. Render all button variants:
   ```tsx
   <Button variant="primary">Primary</Button>
   <Button variant="secondary">Secondary</Button>
   <Button variant="ghost">Ghost</Button>
   ```
2. Verify primary has white background, black text
3. Verify secondary has border, white text
4. Verify ghost has no background, white text
5. Hover over each button
6. Verify shimmer effect appears on hover
7. Click button and verify scale-down on active

**Expected Result**: All variants render correctly with hover effects

---

### Scenario 10: Card Shows Interactive States

**Objective**: Verify Card component responds to hover and active states

**Steps**:
1. Render Card component:
   ```tsx
   <Card onClick={() => {}}>
     <p>Card content</p>
   </Card>
   ```
2. Verify background is #0f0f0f (elevated surface)
3. Verify border is #1a1a1a
4. Verify border radius is 8px
5. Verify padding is 24px
6. Hover over card
7. Verify border brightens on hover
8. Click card
9. Verify glow effect appears when active

**Expected Result**: Card responds to hover (brighten) and active (glow)

---

### Scenario 11: Modal Animates In/Out

**Objective**: Verify Modal component has scale animations

**Steps**:
1. Render Modal with state control:
   ```tsx
   const [isOpen, setIsOpen] = useState(false);
   <Modal isOpen={isOpen} onClose={() => setIsOpen(false)}>
     <p>Modal content</p>
   </Modal>
   ```
2. Open modal (setIsOpen(true))
3. Verify overlay appears with rgba(0, 0, 0, 0.9)
4. Verify backdrop blur is 10px
5. Verify modal scales up on enter
6. Close modal
7. Verify modal scales down on exit

**Expected Result**: Modal animates smoothly with scale transitions

---

### Scenario 12: Layout Regions Have Correct Heights

**Objective**: Verify main layout structure matches specification

**Steps**:
1. Create a layout with all regions:
   ```tsx
   <div className="h-screen flex flex-col">
     <TitleBar {...props} />        {/* 40px */}
     <CommandInput {...props} />    {/* 60px */}
     <div className="flex-1">       {/* flexible */}
       Content Area
     </div>
     <AgentStatusBar {...props} />  {/* 48px */}
     <StatusBar {...props} />       {/* 32px */}
   </div>
   ```
2. Measure each region height
3. Verify TitleBar: 40px
4. Verify CommandInput: 60px
5. Verify AgentStatusBar: 48px
6. Verify StatusBar: 32px
7. Verify ContentArea fills remaining space

**Expected Result**: All regions have correct fixed heights, content area is flexible

---

## Validation Checklist

After completing all scenarios, verify:

- [ ] All design tokens are accessible and correctly valued
- [ ] Glass blue is restricted to 4 branding variants only
- [ ] Typography scale has all 8 levels (48px to 10px)
- [ ] All 4 effects (shimmer, glow, pulse, glass) are defined
- [ ] Spacing follows 8px grid system
- [ ] All 7 components import without errors
- [ ] TitleBar renders with 40px height and correct layout
- [ ] CommandInput shows shimmer + glow on focus
- [ ] Button variants render correctly with hover effects
- [ ] Card responds to hover (brighten) and active (glow)
- [ ] Modal animates with scale transitions
- [ ] Layout regions have correct heights (40, 60, flexible, 48, 32)

## Success Criteria

✅ **PASS**: All 12 scenarios complete successfully  
❌ **FAIL**: Any scenario fails or produces unexpected results

## Troubleshooting

### Tokens not importing
- Check `src/design-system/tokens/index.ts` exports
- Verify TypeScript path alias `@/` is configured

### Components not rendering
- Check React and Framer Motion are installed
- Verify Tailwind CSS is configured
- Check component imports are correct

### Effects not working
- Verify `animations.css` is imported in `globals.css`
- Check Tailwind config includes custom utilities
- Ensure Framer Motion animations are enabled

### Layout heights incorrect
- Check Tailwind height classes are applied
- Verify flex container is full height
- Inspect with browser DevTools

## Next Steps

After successful quickstart validation:
1. Run `/tasks` command to generate implementation tasks
2. Implement components following TDD approach
3. Run full test suite
4. Validate visual appearance matches design spec

---

**Last Updated**: 2025-09-30  
**Status**: Ready for validation
