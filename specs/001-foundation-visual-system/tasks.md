# Tasks: Foundation & Visual System

**Input**: Design documents from `/Users/carsonidsinga/glassflow-lite/specs/001-foundation-visual-system/`
**Prerequisites**: plan.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Branch**: `001-foundation-visual-system`  
**Tech Stack**: TypeScript 5.3+, React 18.2+, Tauri 2.0, Framer Motion 11.0, Tailwind CSS 3.4, Vite 5.0  
**Testing**: Vitest (unit), Playwright (visual regression)

---

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- All paths relative to repository root

---

## Phase 3.1: Setup & Project Structure

- [x] **T001** Create design system directory structure at `src/design-system/` with subdirectories: `tokens/`, `components/`, `styles/`
- [x] **T002** Install testing dependencies: `npm install -D vitest @testing-library/react @testing-library/jest-dom @playwright/test`
- [x] **T003** Configure Vitest in `vitest.config.ts` with React Testing Library setup
- [x] **T004** Configure Playwright in `playwright.config.ts` for visual regression testing
- [x] **T005** Create TypeScript types file at `src/types/design-system.ts` with base interfaces from contracts

---

## Phase 3.2: Design Tokens - Tests First (TDD)

**CRITICAL: These tests MUST be written and MUST FAIL before token implementation**

- [x] **T006** [P] Token validation test in `src/__tests__/design-system/tokens/colors.test.ts` - verify color token structure, hex validation, glass blue restriction
- [x] **T007** [P] Typography token test in `src/__tests__/design-system/tokens/typography.test.ts` - verify 8 type scale levels, font weights 300-700
- [x] **T008** [P] Effect token test in `src/__tests__/design-system/tokens/effects.test.ts` - verify 4 effects (shimmer, glow, pulse, glass) with correct properties
- [x] **T009** [P] Spacing token test in `src/__tests__/design-system/tokens/spacing.test.ts` - verify all values are multiples of 8px (or 4px)

---

## Phase 3.3: Design Tokens - Implementation

**ONLY after T006-T009 tests are failing**

- [x] **T010** [P] Implement color tokens in `src/design-system/tokens/colors.ts` - define monochrome palette + 4 glass blue variants with ColorToken interface
- [x] **T011** [P] Implement typography tokens in `src/design-system/tokens/typography.ts` - define 8-level type scale (hero 48px → tiny 10px) with TypographyToken interface
- [x] **T012** [P] Implement effect tokens in `src/design-system/tokens/effects.ts` - define shimmer (2.5s), glow, pulse (2s), glass (24px blur) with EffectToken interface
- [x] **T013** [P] Implement spacing tokens in `src/design-system/tokens/spacing.ts` - define 8px grid scale with SpacingToken interface
- [x] **T014** Create token exports in `src/design-system/tokens/index.ts` - export all token objects and types

---

## Phase 3.4: Tailwind & Styles Integration

- [x] **T015** Extend Tailwind config in `tailwind.config.js` - import and spread color, typography, spacing tokens into theme.extend
- [x] **T016** Create global styles in `src/design-system/styles/globals.css` - define glass-effect, glass-blue-glow, text-shimmer utilities
- [x] **T017** Create animations CSS in `src/design-system/styles/animations.css` - define @keyframes for shimmer, pulse, glow animations
- [x] **T018** Import styles in main CSS - ensure globals.css and animations.css are imported after Tailwind directives

---

## Phase 3.5: Component Contracts - Tests First (TDD)

**CRITICAL: Component tests MUST be written and MUST FAIL before component implementation**

- [x] **T019** [P] TitleBar contract test in `src/__tests__/design-system/components/TitleBar.test.tsx` - verify props interface, 40px height, layout sections
- [x] **T020** [P] CommandInput contract test in `src/__tests__/design-system/components/CommandInput.test.tsx` - verify props, 60px height, focus effects, Enter submission
- [x] **T021** [P] AgentStatusBar contract test in `src/__tests__/design-system/components/AgentStatusBar.test.tsx` - verify 48px height, 4 agents, hover expansion
- [x] **T022** [P] StatusBar contract test in `src/__tests__/design-system/components/StatusBar.test.tsx` - verify 32px height, 3 sections (left/center/right)
- [x] **T023** [P] Button contract test in `src/__tests__/design-system/components/Button.test.tsx` - verify 3 variants, 4 states, shimmer on hover, scale on active
- [x] **T024** [P] Card contract test in `src/__tests__/design-system/components/Card.test.tsx` - verify 8px radius, 24px padding, border brighten on hover, glow on active
- [x] **T025** [P] Modal contract test in `src/__tests__/design-system/components/Modal.test.tsx` - verify overlay, backdrop blur, scale animations, close handlers

---

## Phase 3.6: Components - Implementation

**ONLY after T019-T025 tests are failing**

- [x] **T026** [P] Implement TitleBar in `src/design-system/components/TitleBar.tsx` - 40px height, logo (left), project name (center), window controls (right), using tokens
- [x] **T027** [P] Implement CommandInput in `src/design-system/components/CommandInput.tsx` - 60px height, 18px font, shimmer + blue glow on focus, Enter key submission
- [x] **T028** [P] Implement AgentStatusBar in `src/design-system/components/AgentStatusBar.tsx` - 48px height, 4 agent indicators, status LED, progress, hover expansion
- [x] **T029** [P] Implement StatusBar in `src/design-system/components/StatusBar.tsx` - 32px height, project path (left), phase (center), time elapsed (right)
- [x] **T030** [P] Implement Button in `src/design-system/components/Button.tsx` - Primary/Secondary/Ghost variants, hover shimmer, active scale-down, disabled state
- [x] **T031** [P] Implement Card in `src/design-system/components/Card.tsx` - Surface background, border, 8px radius, 24px padding, hover brighten, active glow
- [x] **T032** [P] Implement Modal in `src/design-system/components/Modal.tsx` - Overlay with rgba(0,0,0,0.9), 10px backdrop blur, scale in/out animations, close on overlay/escape
- [x] **T033** Create component exports in `src/design-system/components/index.ts` - export all 7 components

---

## Phase 3.7: Integration Tests

- [x] **T034** [P] Integration test in `src/__tests__/design-system/integration/token-tailwind.test.ts` - verify tokens are accessible via Tailwind classes
- [x] **T035** [P] Integration test in `src/__tests__/design-system/integration/component-tokens.test.ts` - verify components correctly use design tokens
- [x] **T036** [P] Integration test in `src/__tests__/design-system/integration/layout-regions.test.tsx` - verify main layout with all 5 regions has correct heights

---

## Phase 3.8: Visual Regression Tests (Playwright)

- [x] **T037** [P] Visual test in `tests/visual/titlebar.spec.ts` - snapshot TitleBar in default state
- [x] **T038** [P] Visual test in `tests/visual/command-input.spec.ts` - snapshot CommandInput in default and focused states
- [x] **T039** [P] Visual test in `tests/visual/buttons.spec.ts` - snapshot all Button variants in all states (default, hover, active, disabled)
- [x] **T040** [P] Visual test in `tests/visual/card.spec.ts` - snapshot Card in default, hover, and active states
- [x] **T041** [P] Visual test in `tests/visual/modal.spec.ts` - snapshot Modal open and close animations

---

## Phase 3.9: Quickstart Validation

- [x] **T042** Execute Scenario 1 from quickstart.md - verify design tokens are accessible
- [x] **T043** Execute Scenario 2 from quickstart.md - verify glass blue restriction (4 variants only)
- [x] **T044** Execute Scenario 3 from quickstart.md - verify typography scale (8 levels)
- [x] **T045** Execute Scenario 4 from quickstart.md - verify effects are defined correctly
- [x] **T046** Execute Scenario 5 from quickstart.md - verify spacing uses 8px grid
- [x] **T047** Execute Scenario 6 from quickstart.md - verify components are importable
- [x] **T048** Execute Scenario 7 from quickstart.md - verify TitleBar renders correctly
- [x] **T049** Execute Scenario 8 from quickstart.md - verify CommandInput focus effects
- [x] **T050** Execute Scenario 9 from quickstart.md - verify Button variants
- [x] **T051** Execute Scenario 10 from quickstart.md - verify Card interactive states
- [x] **T052** Execute Scenario 11 from quickstart.md - verify Modal animations
- [x] **T053** Execute Scenario 12 from quickstart.md - verify layout region heights

---

## Phase 3.10: Polish & Documentation

- [x] **T054** [P] Add JSDoc comments to all token files with usage examples
- [x] **T055** [P] Add JSDoc comments to all component files with prop descriptions
- [x] **T056** [P] Create design system README at `src/design-system/README.md` with import examples and usage guide
- [x] **T057** [P] Update main project README with design system section
- [x] **T058** Run ESLint and fix any linting issues in design system files
- [x] **T059** Run TypeScript compiler in strict mode and fix any type errors
- [x] **T060** Performance audit - verify all animations run at 60fps using Chrome DevTools

---

## Dependencies

### Critical Path
```
Setup (T001-T005)
  ↓
Token Tests (T006-T009) [P]
  ↓
Token Implementation (T010-T014) [P]
  ↓
Tailwind Integration (T015-T018)
  ↓
Component Tests (T019-T025) [P]
  ↓
Component Implementation (T026-T033) [P]
  ↓
Integration Tests (T034-T036) [P]
  ↓
Visual Tests (T037-T041) [P]
  ↓
Quickstart Validation (T042-T053)
  ↓
Polish (T054-T060) [P]
```

### Specific Dependencies
- **T015-T018** require T010-T014 (tokens must exist)
- **T019-T025** require T005, T014 (types and tokens must exist)
- **T026-T033** require T010-T014 (components use tokens)
- **T034-T036** require T026-T033 (integration tests need components)
- **T037-T041** require T026-T033 (visual tests need components)
- **T042-T053** require T026-T033 (quickstart needs full implementation)
- **T054-T060** require all previous tasks

---

## Parallel Execution Examples

### Batch 1: Token Tests (After T005)
```bash
# All token tests can run in parallel - different files, no dependencies
Task T006: "Token validation test in src/__tests__/design-system/tokens/colors.test.ts"
Task T007: "Typography token test in src/__tests__/design-system/tokens/typography.test.ts"
Task T008: "Effect token test in src/__tests__/design-system/tokens/effects.test.ts"
Task T009: "Spacing token test in src/__tests__/design-system/tokens/spacing.test.ts"
```

### Batch 2: Token Implementation (After T006-T009 fail)
```bash
# All token files can be created in parallel - independent files
Task T010: "Implement color tokens in src/design-system/tokens/colors.ts"
Task T011: "Implement typography tokens in src/design-system/tokens/typography.ts"
Task T012: "Implement effect tokens in src/design-system/tokens/effects.ts"
Task T013: "Implement spacing tokens in src/design-system/tokens/spacing.ts"
```

### Batch 3: Component Tests (After T014, T018)
```bash
# All component tests can run in parallel - different files
Task T019: "TitleBar contract test in src/__tests__/design-system/components/TitleBar.test.tsx"
Task T020: "CommandInput contract test in src/__tests__/design-system/components/CommandInput.test.tsx"
Task T021: "AgentStatusBar contract test in src/__tests__/design-system/components/AgentStatusBar.test.tsx"
Task T022: "StatusBar contract test in src/__tests__/design-system/components/StatusBar.test.tsx"
Task T023: "Button contract test in src/__tests__/design-system/components/Button.test.tsx"
Task T024: "Card contract test in src/__tests__/design-system/components/Card.test.tsx"
Task T025: "Modal contract test in src/__tests__/design-system/components/Modal.test.tsx"
```

### Batch 4: Component Implementation (After T019-T025 fail)
```bash
# All components can be implemented in parallel - independent files
Task T026: "Implement TitleBar in src/design-system/components/TitleBar.tsx"
Task T027: "Implement CommandInput in src/design-system/components/CommandInput.tsx"
Task T028: "Implement AgentStatusBar in src/design-system/components/AgentStatusBar.tsx"
Task T029: "Implement StatusBar in src/design-system/components/StatusBar.tsx"
Task T030: "Implement Button in src/design-system/components/Button.tsx"
Task T031: "Implement Card in src/design-system/components/Card.tsx"
Task T032: "Implement Modal in src/design-system/components/Modal.tsx"
```

### Batch 5: Visual Tests (After T033)
```bash
# All visual tests can run in parallel - independent test files
Task T037: "Visual test in tests/visual/titlebar.spec.ts"
Task T038: "Visual test in tests/visual/command-input.spec.ts"
Task T039: "Visual test in tests/visual/buttons.spec.ts"
Task T040: "Visual test in tests/visual/card.spec.ts"
Task T041: "Visual test in tests/visual/modal.spec.ts"
```

---

## Notes

### TDD Enforcement
- **Token tests (T006-T009)** must fail before implementing tokens (T010-T014)
- **Component tests (T019-T025)** must fail before implementing components (T026-T033)
- Run tests after each implementation task to verify they pass

### Parallel Execution
- Tasks marked **[P]** can run simultaneously
- Only execute [P] tasks in parallel if they're in the same batch
- Never parallelize tasks that modify the same file

### Commit Strategy
- Commit after each completed task
- Commit message format: `feat(design-system): [Task ID] - [Description]`
- Example: `feat(design-system): T010 - Implement color tokens`

### Testing Commands
```bash
# Run unit tests
npm run test

# Run specific test file
npm run test src/__tests__/design-system/tokens/colors.test.ts

# Run visual regression tests
npx playwright test

# Run all tests
npm run test && npx playwright test
```

---

## Validation Checklist

**Before marking feature complete, verify:**

- [x] All contracts (tokens.ts, components.ts) have corresponding tests
- [x] All entities (4 token types, 7 components) have implementation tasks
- [x] All tests come before implementation (TDD enforced)
- [x] Parallel tasks are truly independent (different files)
- [x] Each task specifies exact file path
- [x] No [P] task modifies same file as another [P] task
- [x] All 12 quickstart scenarios have validation tasks
- [x] Dependencies are clearly documented
- [x] Estimated 60 tasks total (matches scope)

---

## Task Execution Status

**Total Tasks**: 60  
**Completed**: 60  
**In Progress**: 0  
**Pending**: 0

**Estimated Time**: 20-25 hours (with parallel execution)
**Actual Time**: Completed in single session

## ✅ IMPLEMENTATION COMPLETE

All 60 tasks have been successfully completed. The Foundation & Visual System is fully implemented and ready for use.

---

**Ready for implementation!** Start with T001 and follow the dependency graph.
