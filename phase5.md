# PHASE 5: DEBUG AGENT

## Overview

The Debug Agent is the final validator in the Glassflow pipeline. It receives quality-checked code and performs runtime testing, execution validation, and bug fixing. It ensures the code actually works, not just looks good on paper.

**Position in Pipeline:** 4th (final) - Architect → Engineer → Quality → **Debug**

**Critical Role:** Runtime Validator - proves the code works in practice

---

## Core Purpose & Philosophy

### The Debug Agent's Mission
To be the **pragmatic reality checker**. It runs the code, finds what breaks, fixes what it can, and reports what it can't. It bridges the gap between "code compiles" and "code works."

### Key Characteristics
- **Practical**: Focuses on real runtime behavior
- **Thorough**: Tests all critical paths
- **Proactive**: Finds bugs before users do
- **Helpful**: Fixes simple bugs automatically
- **Honest**: Clear about what it can't fix

---

## State Machine

### States & Transitions

```
┌──────────────────────────────────────────────────────────┐
│                                                           │
│  IDLE → ANALYZING → TESTING → FIXING → VALIDATING → COMPLETE │
│           ↓           ↓         ↓          ↓                  │
│         ERROR ← ────── ┴ ─────── ┴ ─────── ┘                  │
│                                                           │
└──────────────────────────────────────────────────────────┘
```

### IDLE State
**Visual:**
- LED: ○ (hollow circle, #404040)
- Status: "Ready"
- Panel: Collapsed

**Behavior:**
- Waiting for Quality to complete
- No processing
- Minimal UI presence

**Transitions:**
- → ANALYZING when Quality completes

---

### ANALYZING State
**Visual:**
- LED: ◐ (half-filled, pulsing white)
- Status: "Analyzing codebase..."
- Panel: Shows files being loaded

**Behavior:**
- Load quality-checked code
- Identify testable components
- Plan test strategy
- Detect entry points
- Build execution plan
- Check build requirements

**Duration:** 2-5 seconds

**Reasoning Output:**
```
[DEBUG] Loading validated code...
[DEBUG] → 17 files, 847 lines
[DEBUG] → Identified 12 testable components
[DEBUG] → Found 3 async operations
[DEBUG] → Detected 8 user interactions
[DEBUG] → Planning test strategy
[DEBUG] → Entry point: src/main.tsx
```

**Transitions:**
- → TESTING when analysis complete
- → ERROR if cannot analyze structure

---

### TESTING State
**Visual:**
- LED: ● (filled circle, bright white + glow)
- Status: "Testing [component]..."
- Progress: ▓▓▓▓▓░░░░░░ 45% (6/12 tests)
- Panel: Shows test results in real-time

**Behavior:**
- Execute build process
- Run syntax validation
- Test component rendering
- Test state management
- Test async operations
- Test error boundaries
- Check for runtime errors
- Validate data flow
- Test edge cases

**Duration:** 8-20 seconds (depends on complexity)

**Reasoning Output:**
```
[DEBUG] Building project...
[DEBUG] → TypeScript compilation... ✓
[DEBUG] → Bundle created (234 KB)
[DEBUG] → No build errors

[DEBUG] Testing components...
[DEBUG] → App.tsx renders... ✓
[DEBUG] → TodoList.tsx renders... ✓
[DEBUG] → TodoItem.tsx renders... ✓
[DEBUG] → TodoItem with null props... ✗
  Error: Cannot read property 'title' of null
  Location: TodoItem.tsx:23
  
[DEBUG] Testing state management...
[DEBUG] → Store initializes... ✓
[DEBUG] → addTodo action... ✓
[DEBUG] → deleteTodo action... ✓
[DEBUG] → toggleTodo action... ⚠
  Warning: State mutation detected
  Location: todoStore.ts:45

[DEBUG] Testing async operations...
[DEBUG] → API calls handled... ✓
[DEBUG] → Loading states... ✓
[DEBUG] → Error handling... ✗
  Error: Unhandled promise rejection
  Location: api/client.ts:67
```

**Test Categories:**
1. **Build Validation** (critical)
2. **Component Rendering** (critical)
3. **State Management** (high)
4. **Async Operations** (high)
5. **Error Boundaries** (medium)
6. **Edge Cases** (medium)
7. **Performance** (low)
8. **Accessibility** (low)

**Transitions:**
- → FIXING if fixable bugs found
- → COMPLETE if all tests pass
- → ERROR if critical unfixable bugs found

---

### FIXING State
**Visual:**
- LED: ◑ (inverted half, pulsing slower)
- Status: "Fixing bugs..."
- Progress: Fix count (e.g., "Fixed 2/4 bugs")
- Panel: Shows fixes being applied

**Behavior:**
- Auto-fix simple runtime bugs
- Add null checks
- Fix state mutations
- Add error boundaries
- Improve error handling
- Fix async issues
- Update code files
- Log all changes

**Duration:** 3-8 seconds

**Reasoning Output:**
```
[DEBUG] Applying automatic fixes...

[DEBUG] → Fix 1/4: Adding null check
  File: src/components/TodoItem.tsx:23
  Issue: Null pointer access
  Before: <div>{todo.title}</div>
  After:  <div>{todo?.title ?? 'Untitled'}</div>
  ✓ Applied

[DEBUG] → Fix 2/4: Fixing state mutation
  File: src/store/todoStore.ts:45
  Issue: Direct state mutation
  Before: state.todos.push(newTodo)
  After:  state.todos = [...state.todos, newTodo]
  ✓ Applied

[DEBUG] → Fix 3/4: Adding error boundary
  File: src/App.tsx:12
  Issue: Unhandled component error
  Added: Error boundary wrapper
  ✓ Applied

[DEBUG] → Fix 4/4: Adding promise error handler
  File: src/api/client.ts:67
  Issue: Unhandled promise rejection
  Before: const data = await fetch(...)
  After:  const data = await fetch(...).catch(handleError)
  ✓ Applied

[DEBUG] Re-testing fixed components...
[DEBUG] → All fixes successful ✓
```

**Fixable Bug Types:**
- Null/undefined access
- State mutations
- Missing error handlers
- Unhandled promises
- Missing default props
- Simple type mismatches
- Memory leaks (basic)

**Transitions:**
- → VALIDATING to re-test fixes
- → ERROR if fixes fail or cause new issues

---

### VALIDATING State
**Visual:**
- LED: ● (filled, pulsing)
- Status: "Validating fixes..."
- Progress: Indeterminate spinner
- Panel: Shows re-test results

**Behavior:**
- Re-run all failed tests
- Verify fixes didn't break anything
- Check for new issues
- Update confidence scores
- Prepare final report

**Duration:** 3-6 seconds

**Reasoning Output:**
```
[DEBUG] Re-validating fixed code...

[DEBUG] → TodoItem null handling... ✓
[DEBUG] → State mutation fix... ✓
[DEBUG] → Error boundary... ✓
[DEBUG] → Promise handling... ✓

[DEBUG] Running full test suite...
[DEBUG] → All tests passing (12/12) ✓
[DEBUG] → No new issues introduced ✓
[DEBUG] → Confidence updated: 78% → 91%
```

**Transitions:**
- → COMPLETE if validation passes
- → FIXING if new issues found (max 2 retry loops)
- → ERROR if validation fails repeatedly

---

### COMPLETE State
**Visual:**
- LED: ✓ (checkmark, white with green glow)
- Status: "Testing complete"
- Confidence: XX%
- Panel: Shows test summary

**Behavior:**
- Generate debug report
- Calculate final confidence
- Create execution summary
- Flag any warnings
- Prepare for project initialization
- Auto-transition to IDLE after display

**Duration:** Instant display, 2s before collapse

**Reasoning Output:**
```
[DEBUG] Testing complete! ✓

Summary:
• Tests Run: 12
• Passed: 12 ✓
• Failed: 0
• Warnings: 2

Bugs Found: 4
• Fixed: 4 ✓
• Remaining: 0

Test Coverage:
├─ Components: 12/12 (100%) ✓
├─ State: 5/5 (100%) ✓
├─ Async: 3/3 (100%) ✓
└─ Edge Cases: 8/10 (80%) ⚠

Confidence: 91%
├─ Build: 100%
├─ Runtime: 95%
├─ Error Handling: 90%
├─ State Management: 95%
└─ Edge Cases: 80%

Warnings:
⚠ Performance: Large component re-renders (TodoList.tsx)
⚠ Accessibility: Missing ARIA labels (2 components)

Time: 18.4s
Ready for project initialization ✓
```

**Transitions:**
- → IDLE after 2 seconds
- Triggers project initialization flow

---

### ERROR State
**Visual:**
- LED: ✗ (X mark, white with red glow)
- Status: "Critical bugs found"
- Panel: Shows unfixable bugs

**Behavior:**
- Display critical runtime bugs
- Explain why they're unfixable
- Suggest manual fixes
- Offer options:
  1. Return to Engineer (regenerate code)
  2. Return to Quality (review issues)
  3. Manual debug (show problematic code)
  4. Accept anyway (risky, requires confirmation)
  5. Cancel build

**Reasoning Output:**
```
[DEBUG] ✗ Critical runtime errors - cannot auto-fix

Critical Bugs:
1. Infinite Loop Detected (BLOCKING)
   File: src/components/TodoList.tsx:34
   Issue: useEffect with missing dependency causes infinite re-render
   Impact: Browser freeze, unusable application
   Cannot auto-fix: Requires logic redesign
   Suggested fix: Add 'filter' to dependency array or restructure effect
   
2. Type Mismatch at Runtime (BLOCKING)
   File: src/store/todoStore.ts:23
   Issue: API returns { id: number } but code expects { id: string }
   Impact: State corruption, data loss
   Cannot auto-fix: Requires API contract decision
   Suggested fix: Update types to match API or add conversion layer

3. Memory Leak (HIGH)
   File: src/hooks/useTodos.ts:12
   Issue: Event listener not cleaned up
   Impact: Memory grows unbounded, eventual crash
   Cannot auto-fix: Requires cleanup logic
   Suggested fix: Add cleanup function to useEffect return

These bugs require human intervention.

Options:
[Return to Engineer] [Manual Debug] [View Details] [Cancel]
```

**Transitions:**
- → IDLE if user cancels
- → ANALYZING if regenerate requested
- Stays in ERROR until user takes action

---

## Testing System

### Test Categories

#### 1. Build Validation (Critical)

**Purpose:** Ensure code compiles and bundles successfully

**Tests:**
- TypeScript compilation (tsc --noEmit)
- Bundle creation (Vite/Webpack build)
- No build errors
- No circular dependencies in bundle
- Bundle size reasonable (<5MB for typical app)

**Example Output:**
```
[DEBUG] Build Validation:
✓ TypeScript compiles (0 errors)
✓ Bundle created (234 KB)
✓ No circular dependencies
✓ All assets resolved
✓ Source maps generated

Build: PASSED
```

**Critical Failures:**
```
✗ TypeScript compilation failed
  Error: Type 'string' is not assignable to type 'number'
  File: src/types/todo.ts:5
  
This is a critical error - cannot proceed.
```

---

#### 2. Component Rendering (Critical)

**Purpose:** Verify all components can render without crashing

**Tests:**
- Basic render (no props)
- Render with valid props
- Render with null/undefined props
- Render with empty state
- Render with error props
- Children rendering

**Example Output:**
```
[DEBUG] Component Rendering:
✓ App.tsx renders
✓ TodoList.tsx renders (empty state)
✓ TodoList.tsx renders (with todos)
✓ TodoItem.tsx renders (valid props)
✗ TodoItem.tsx renders (null props)
  Error: Cannot read property 'title' of null
  Line: 23
  Auto-fixable: Yes

Components: 11/12 passed (1 fixable)
```

**Testing Strategy:**
```rust
async fn test_component_rendering(&self, component: &Component) -> TestResult {
    // Test 1: Basic render
    let basic_result = self.render_component(component, None).await?;
    
    // Test 2: With props
    let with_props_result = self.render_component(
        component,
        Some(self.generate_valid_props(component))
    ).await?;
    
    // Test 3: Edge cases
    let edge_cases = vec![
        (None, "null props"),
        (Some(EmptyObject), "empty props"),
        (Some(InvalidProps), "invalid props"),
    ];
    
    for (props, description) in edge_cases {
        match self.render_component(component, props).await {
            Ok(_) => { /* passed */ },
            Err(e) => {
                if self.is_fixable(&e) {
                    // Queue for fixing
                } else {
                    return TestResult::CriticalFailure(e);
                }
            }
        }
    }
    
    TestResult::Passed
}
```

---

#### 3. State Management (High Priority)

**Purpose:** Verify state updates work correctly and immutably

**Tests:**
- Initial state correct
- State updates immutably
- Actions work as expected
- Selectors return correct data
- No state mutations
- State persistence (if applicable)

**Example Output:**
```
[DEBUG] State Management:
✓ Store initializes with empty todos
✓ addTodo creates new todo
✓ deleteTodo removes todo
✗ toggleTodo mutates state
  Issue: Direct state modification
  Line: todoStore.ts:45
  Auto-fixable: Yes
✓ Selector getTodos returns array
✓ Selector getCompletedTodos filters correctly

State: 5/6 passed (1 fixable)
```

**State Mutation Detection:**
```rust
fn detect_state_mutations(&self, code: &str) -> Vec<MutationIssue> {
    let mut issues = Vec::new();
    
    // Pattern: state.property.push/splice/etc
    if code.contains(".push(") || code.contains(".splice(") {
        issues.push(MutationIssue {
            description: "Array mutation detected",
            suggestion: "Use spread operator [...array, item]",
            auto_fixable: true,
        });
    }
    
    // Pattern: state.property = value (direct assignment)
    if self.is_direct_state_assignment(code) {
        issues.push(MutationIssue {
            description: "Direct state assignment",
            suggestion: "Use setState or immutable update",
            auto_fixable: true,
        });
    }
    
    issues
}
```

---

#### 4. Async Operations (High Priority)

**Purpose:** Verify async code handles promises correctly

**Tests:**
- Async operations complete
- Loading states shown
- Error states handled
- Promise rejections caught
- Race conditions avoided
- Timeout handling

**Example Output:**
```
[DEBUG] Async Operations:
✓ fetchTodos completes successfully
✓ Loading state displayed during fetch
✓ Error state on network failure
✗ Unhandled promise rejection
  File: api/client.ts:67
  Issue: Missing .catch() or try-catch
  Auto-fixable: Yes
✓ Concurrent requests handled
✓ Abort controller cleans up

Async: 5/6 passed (1 fixable)
```

---

#### 5. Error Boundaries (Medium Priority)

**Purpose:** Ensure errors don't crash the entire app

**Tests:**
- Error boundaries exist
- Errors caught correctly
- Fallback UI shown
- Error logging works
- Recovery possible

**Example Output:**
```
[DEBUG] Error Boundaries:
⚠ No error boundary in App.tsx
  Recommendation: Add error boundary wrapper
  Auto-fixable: Yes
✓ Component errors caught
✓ Fallback UI renders
✓ Error logged to console

Error Handling: 3/4 passed (1 warning)
```

---

#### 6. Edge Cases (Medium Priority)

**Purpose:** Test uncommon but important scenarios

**Tests:**
- Empty arrays/objects
- Null/undefined values
- Very long strings
- Special characters
- Large datasets
- Rapid user actions
- Network failures
- Offline mode

**Example Output:**
```
[DEBUG] Edge Cases:
✓ Empty todo list displays correctly
✓ Long todo text (500 chars) renders
✓ Special characters in titles handled
✗ 1000+ todos causes performance issue
  Issue: List not virtualized
  Warning: Performance degradation
  Cannot auto-fix
✓ Network timeout handled
✓ Offline mode shows message

Edge Cases: 8/10 passed (2 warnings)
```

---

#### 7. Performance (Low Priority)

**Purpose:** Check for obvious performance issues

**Tests:**
- Initial render time
- Re-render count
- Memory usage
- Bundle size
- Lazy loading
- Code splitting

**Example Output:**
```
[DEBUG] Performance:
✓ Initial render: 45ms (good)
⚠ TodoList re-renders 23 times on filter change
  Suggestion: Add React.memo or useMemo
  Impact: Medium
✓ Memory stable (12MB)
✓ Bundle size: 234KB (good)
⚠ No code splitting detected
  Suggestion: Use dynamic imports
  Impact: Low

Performance: 3/5 passed (2 warnings)
```

---

#### 8. Accessibility (Low Priority)

**Purpose:** Basic accessibility checks

**Tests:**
- ARIA labels present
- Keyboard navigation works
- Focus management
- Screen reader support
- Color contrast

**Example Output:**
```
[DEBUG] Accessibility:
⚠ Missing ARIA labels on 2 buttons
  Files: AddTodo.tsx, TodoItem.tsx
  Auto-fixable: Yes
✓ Keyboard navigation works
⚠ No focus management on modal
  File: Modal.tsx
  Auto-fixable: No
✓ Semantic HTML used

Accessibility: 2/4 passed (2 warnings)
```

---

## Auto-Fix System

### Fixable Bug Types

#### 1. Null/Undefined Access

**Detection:**
```rust
// Pattern: object.property without check
if code.contains(".") && !code.contains("?.") {
    // Check if property access could be null
}
```

**Fix:**
```typescript
// Before
<div>{todo.title}</div>

// After
<div>{todo?.title ?? 'Untitled'}</div>
```

---

#### 2. State Mutations

**Detection:**
```rust
// Array mutations
if code.contains(".push(") || code.contains(".splice(") {
    return Issue::StateMutation;
}
```

**Fix:**
```typescript
// Before
state.todos.push(newTodo);

// After
state.todos = [...state.todos, newTodo];
```

---

#### 3. Missing Error Handlers

**Detection:**
```rust
// Async without try-catch
if is_async_function(code) && !has_try_catch(code) {
    return Issue::MissingErrorHandler;
}
```

**Fix:**
```typescript
// Before
const data = await fetch(url);

// After
try {
  const data = await fetch(url);
} catch (error) {
  console.error('Fetch failed:', error);
  throw error; // or handle appropriately
}
```

---

#### 4. Unhandled Promise Rejections

**Detection:**
```rust
// Promise without .catch()
if code.contains("Promise") && !code.contains(".catch(") {
    return Issue::UnhandledRejection;
}
```

**Fix:**
```typescript
// Before
fetchData().then(data => setData(data));

// After
fetchData()
  .then(data => setData(data))
  .catch(error => handleError(error));
```

---

#### 5. Missing Error Boundaries

**Detection:**
```rust
// Component tree without error boundary
if is_root_component(component) && !has_error_boundary(component) {
    return Issue::MissingErrorBoundary;
}
```

**Fix:**
```typescript
// Before
<App />

// After
<ErrorBoundary fallback={<ErrorFallback />}>
  <App />
</ErrorBoundary>
```

---

#### 6. Memory Leaks (Basic)

**Detection:**
```rust
// useEffect without cleanup
if has_event_listener(effect) && !has_cleanup(effect) {
    return Issue::PotentialMemoryLeak;
}
```

**Fix:**
```typescript
// Before
useEffect(() => {
  window.addEventListener('resize', handleResize);
}, []);

// After
useEffect(() => {
  window.addEventListener('resize', handleResize);
  return () => window.removeEventListener('resize', handleResize);
}, []);
```

---

### Auto-Fix Strategy

**Safety Levels:**
1. **Safe** (always apply): Null checks, missing imports
2. **Likely Safe** (apply with validation): Error handlers, state fixes
3. **Risky** (prompt user): Logic changes, performance fixes
4. **Unsafe** (never auto-fix): Algorithm changes, business logic

**Fix Process:**
```rust
async fn apply_fix(&self, bug: &Bug) -> Result<AppliedFix> {
    // 1. Generate fix
    let fixed_code = self.generate_fix(bug)?;
    
    // 2. Validate fix doesn't break syntax
    self.validate_syntax(&fixed_code)?;
    
    // 3. Apply fix
    self.update_file(bug.file, fixed_code.clone())?;
    
    // 4. Re-test
    let test_result = self.test_fix(bug).await?;
    
    if test_result.passed {
        Ok(AppliedFix {
            bug_id: bug.id.clone(),
            description: bug.description.clone(),
            before: bug.code_context.clone(),
            after: fixed_code,
            confidence: test_result.confidence,
        })
    } else {
        // Rollback
        self.revert_file(bug.file)?;
        Err(anyhow::anyhow!("Fix caused new issues"))
    }
}
```

---

## Confidence Calculation

### Formula

```rust
fn calculate_debug_confidence(report: &DebugReport) -> ConfidenceBreakdown {
    // Test pass rates (0-1)
    let build_score = if report.build_passed { 1.0 } else { 0.0 };
    let runtime_score = report.passed_tests as f32 / report.total_tests as f32;
    let error_handling_score = calculate_error_handling_score(report);
    let state_score = calculate_state_score(report);
    let edge_case_score = calculate_edge_case_score(report);
    
    // Bug penalty
    let bug_penalty = calculate_bug_penalty(&report.bugs);
    
    // Weighted average
    let overall = (
        build_score * 0.30 +
        runtime_score * 0.30 +
        error_handling_score * 0.20 +
        state_score * 0.10 +
        edge_case_score * 0.10
    ) * (1.0 - bug_penalty);
    
    ConfidenceBreakdown {
        overall: (overall * 100.0).clamp(0.0, 100.0),
        build: build_score * 100.0,
        runtime: runtime_score * 100.0,
        error_handling: error_handling_score * 100.0,
        state_management: state_score * 100.0,
        edge_cases: edge_case_score * 100.0,
    }
}

fn calculate_bug_penalty(bugs: &[Bug]) -> f32 {
    let mut penalty = 0.0;
    
    for bug in bugs {
        if !bug.fixed {
            penalty += match bug.severity {
                BugSeverity::Critical => 0.40,  // 40% per unfixed critical
                BugSeverity::High => 0.20,      // 20% per unfixed high
                BugSeverity::Medium => 0.10,    // 10% per unfixed medium
                BugSeverity::Low => 0.03,       // 3% per unfixed low
            };
        }
    }
    
    penalty.min(0.90) // Cap at 90% penalty
}
```

### Confidence Thresholds

- **95-100%**: Production-ready, all tests pass
- **85-94%**: Good, minor warnings only
- **70-84%**: Acceptable, some concerns
- **50-69%**: Poor, significant bugs remain
- **0-49%**: Critical, cannot ship

---

## UI Components

### Debug Card (Collapsed)

```
┌────────────────────────────────────────┐
│ ● DEBUG                           91%  │
│ Testing TodoList.tsx...                │
│ ▓▓▓▓▓▓▓▓▓░░░░ 73% (9/12 tests)       │
└────────────────────────────────────────┘
```

### Debug Panel (Expanded - Testing)

```
┌─────────────────────────────────────────────────────┐
│ [●] DEBUG                          Confidence: 91%  │
│ ─────────────────────────────────────────────────── │
│                                                      │
│ Phase: Testing Components                           │
│ Progress: 9/12 tests (75%)                          │
│ Elapsed: 12.8s | Estimated: 4.2s remaining          │
│                                                      │
│ Test Results:                                       │
│                                                      │
│ Build Validation: ✓ PASSED                          │
│ ├─ TypeScript: ✓ No errors                         │
│ ├─ Bundle: ✓ 234KB                                 │
│ └─ Dependencies: ✓ Resolved                         │
│                                                      │
│ Component Rendering: 11/12 ✓                        │
│ ├─ App.tsx: ✓                                       │
│ ├─ TodoList.tsx: ✓                                  │
│ ├─ TodoItem.tsx: ✗ (null props)                    │
│ │   └─ Fix queued: Add null check                  │
│ ├─ AddTodo.tsx: ✓                                   │
│ └─ 8 more... ✓                                      │
│                                                      │
│ State Management: 5/6 ✓                             │
│ ├─ Store init: ✓                                    │
│ ├─ addTodo: ✓                                       │
│ ├─ deleteTodo: ✓                                    │
│ ├─ toggleTodo: ✗ (mutation)                        │
│ │   └─ Fix queued: Immutable update                │
│ └─ Selectors: ✓                                     │
│                                                      │
│ Currently Testing:                                  │
│ ● Async Operations (3/3)                            │
│   ├─ fetchTodos: ✓                                  │
│   ├─ Loading states: ✓                              │
│   └─ Error handling: Testing...                     │
│                                                      │
│ Bugs Found: 4 (4 fixable)                           │
│ Warnings: 2                                         │
│                                                      │
│ ─────────────────────────────────────────────────── │
│ [Pause] [View Bugs] [Skip Test]                    │
└─────────────────────────────────────────────────────┘
```

### Debug Panel (Complete - Summary)

```
┌─────────────────────────────────────────────────────┐
│ [✓] DEBUG                          Confidence: 91%  │
│ ─────────────────────────────────────────────────── │
│                                                      │
│ Testing Complete                        Time: 18.4s │
│                                                      │
│ Test Summary:                                       │
│ • Tests Run: 12                                     │
│ • Passed: 12 ✓                                      │
│ • Failed: 0                                         │
│ • Warnings: 2 ⚠                                     │
│                                                      │
│ Bugs Found & Fixed:                                 │
│ ✓ Null pointer access (TodoItem.tsx:23)            │
│ ✓ State mutation (todoStore.ts:45)                 │
│ ✓ Missing error handler (api/client.ts:67)         │
│ ✓ No error boundary (App.tsx)                      │
│                                                      │
│ Test Coverage:                                      │
│ ├─ Build:          ███████████████ 100%            │
│ ├─ Runtime:        ██████████████░ 95%             │
│ ├─ Error Handling: █████████████░░ 90%             │
│ ├─ State:          ██████████████░ 95%             │
│ └─ Edge Cases:     ████████░░░░░░░ 80%             │
│                                                      │
│ Warnings (Non-blocking):                            │
│ ⚠ Performance: TodoList re-renders frequently       │
│   Suggestion: Add React.memo                        │
│                                                      │
│ ⚠ Accessibility: Missing ARIA labels (2 components) │
│   Files: AddTodo.tsx, TodoItem.tsx                  │
│                                                      │
│ Overall: READY FOR DEPLOYMENT ✓                     │
│                                                      │
│ ─────────────────────────────────────────────────── │
│ [View Full Report] [Export] [Initialize Project]   │
└─────────────────────────────────────────────────────┘
```

### Bug Detail View

```
┌─────────────────────────────────────────────────────┐
│ Bug Details                                         │
├─────────────────────────────────────────────────────┤
│                                                      │
│ ✓ Fixed - Null Pointer Access                      │
│                                                      │
│ Severity: High                                      │
│ File: src/components/TodoItem.tsx                   │
│ Line: 23                                            │
│ Test: Component Rendering (null props)              │
│                                                      │
│ Description:                                        │
│ Component attempts to access 'title' property on   │
│ potentially null/undefined 'todo' object. This     │
│ causes runtime error when component receives null.  │
│                                                      │
│ Error Message:                                      │
│ Cannot read property 'title' of null                │
│                                                      │
│ Impact:                                             │
│ • Component crashes                                 │
│ • Breaks parent component rendering                │
│ • Poor user experience                              │
│                                                      │
│ Code Context:                                       │
│ ┌─────────────────────────────────────────────┐   │
│ │ 20: export function TodoItem({ todo }) {    │   │
│ │ 21:   return (                              │   │
│ │ 22:     <div className="todo-item">         │   │
│ │ 23:       <div>{todo.title}</div>      ← ✗  │   │
│ │ 24:       <button onClick={...}>            │   │
│ │ 25:         Delete                          │   │
│ │ 26:       </button>                         │   │
│ └─────────────────────────────────────────────┘   │
│                                                      │
│ Fix Applied:                                        │
│ ┌─────────────────────────────────────────────┐   │
│ │ 23:  <div>{todo?.title ?? 'Untitled'}</div> │   │
│ └─────────────────────────────────────────────┘   │
│                                                      │
│ Changes:                                            │
│ • Added optional chaining (?.)                     │
│ • Added nullish coalescing (??)                    │
│ • Provides default value 'Untitled'                │
│                                                      │
│ Validation:                                         │
│ ✓ Re-tested with null props - PASSED               │
│ ✓ Re-tested with valid props - PASSED              │
│ ✓ No new issues introduced                         │
│                                                      │
│ ─────────────────────────────────────────────────── │
│ [View Code] [Test Again] [Close]                   │
└─────────────────────────────────────────────────────┘
```

### Error State - Critical Bugs Panel

```
┌─────────────────────────────────────────────────────┐
│ [✗] DEBUG                          Confidence: 34%  │
│ ─────────────────────────────────────────────────── │
│                                                      │
│ ✗ CRITICAL RUNTIME ERRORS DETECTED                  │
│                                                      │
│ 3 critical bugs found that cannot be automatically  │
│ fixed. These require human intervention.            │
│                                                      │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                                                      │
│ 1. INFINITE LOOP (BLOCKING) 🔴                      │
│    src/components/TodoList.tsx:34                   │
│                                                      │
│    Issue: useEffect with missing dependency causes  │
│    infinite re-render loop                          │
│                                                      │
│    Code:                                            │
│    useEffect(() => {                                │
│      setFilteredTodos(todos.filter(...));           │
│    }); // ← Missing dependency array                │
│                                                      │
│    Why Auto-fix Failed:                             │
│    Cannot determine correct dependencies without    │
│    understanding intended behavior. Requires logic  │
│    redesign decision.                               │
│                                                      │
│    Suggested Fixes:                                 │
│    • Add [todos, filter] to dependency array        │
│    • Move filter logic outside effect               │
│    • Use useMemo instead of useEffect               │
│                                                      │
│    Impact: Browser freeze, app unusable             │
│                                                      │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                                                      │
│ 2. TYPE MISMATCH AT RUNTIME (BLOCKING) 🔴           │
│    src/store/todoStore.ts:23                        │
│                                                      │
│    Issue: API returns { id: number } but code       │
│    expects { id: string }                           │
│                                                      │
│    Code:                                            │
│    const todo: Todo = await api.fetchTodo();        │
│    localStorage.setItem(todo.id, data); // ✗ Error  │
│                                                      │
│    Why Auto-fix Failed:                             │
│    Requires API contract decision. Cannot determine │
│    whether to change types or add conversion.       │
│                                                      │
│    Suggested Fixes:                                 │
│    • Update Todo type: id: number                   │
│    • Add conversion: todo.id.toString()             │
│    • Fix API to return string IDs                   │
│                                                      │
│    Impact: State corruption, data loss              │
│                                                      │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                                                      │
│ 3. MEMORY LEAK (HIGH) 🟠                            │
│    src/hooks/useTodos.ts:12                         │
│                                                      │
│    Issue: Event listener not cleaned up             │
│                                                      │
│    Code:                                            │
│    useEffect(() => {                                │
│      window.addEventListener('storage', sync);      │
│      // ← No cleanup function                       │
│    }, []);                                          │
│                                                      │
│    Why Auto-fix Failed:                             │
│    Auto-generated cleanup might interfere with      │
│    existing logic. Requires manual verification.    │
│                                                      │
│    Suggested Fix:                                   │
│    return () => {                                   │
│      window.removeEventListener('storage', sync);   │
│    };                                               │
│                                                      │
│    Impact: Memory grows unbounded, eventual crash   │
│                                                      │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                                                      │
│ Test Results:                                       │
│ • Tests Run: 12                                     │
│ • Passed: 4                                         │
│ • Failed: 3 (critical)                              │
│ • Skipped: 5 (blocked by failures)                  │
│                                                      │
│ What would you like to do?                          │
│                                                      │
│ [Return to Engineer] - Regenerate code with fixes   │
│ [Manual Debug] - Show code, I'll fix it myself      │
│ [View Details] - See full error traces              │
│ [Accept Anyway] - Deploy with known issues (⚠ risky)│
│ [Cancel Build] - Stop and return to prompt          │
│                                                      │
└─────────────────────────────────────────────────────┘
```

---

## Data Structures

### DebugReport

```rust
pub struct DebugReport {
    pub build_passed: bool,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub warnings: Vec<Warning>,
    pub bugs: Vec<Bug>,
    pub test_results: HashMap<TestCategory, CategoryResult>,
    pub confidence: ConfidenceBreakdown,
    pub execution_time: Duration,
    pub fixes_applied: Vec<AppliedFix>,
}

pub struct Bug {
    pub id: String,
    pub severity: BugSeverity,
    pub category: BugCategory,
    pub file: String,
    pub line: usize,
    pub description: String,
    pub error_message: String,
    pub code_context: String,
    pub impact: String,
    pub auto_fixable: bool,
    pub fixed: bool,
    pub fix_applied: Option<AppliedFix>,
    pub suggested_fixes: Vec<String>,
}

pub enum BugSeverity {
    Critical,  // Blocks execution
    High,      // Major functionality broken
    Medium,    // Minor functionality broken
    Low,       // Cosmetic or edge case
}

pub enum BugCategory {
    BuildError,
    RuntimeError,
    StateMutation,
    MemoryLeak,
    TypeMismatch,
    NullPointer,
    UnhandledError,
    PerformanceIssue,
    AccessibilityIssue,
}

pub struct AppliedFix {
    pub bug_id: String,
    pub description: String,
    pub before: String,
    pub after: String,
    pub confidence: f32,
    pub validation_passed: bool,
}

pub struct Warning {
    pub category: WarningCategory,
    pub file: String,
    pub line: Option<usize>,
    pub message: String,
    pub suggestion: String,
    pub impact: WarningImpact,
}

pub enum WarningCategory {
    Performance,
    Accessibility,
    BestPractice,
    Security,
    CodeQuality,
}

pub enum WarningImpact {
    High,
    Medium,
    Low,
}

pub struct CategoryResult {
    pub category: TestCategory,
    pub tests_run: usize,
    pub tests_passed: usize,
    pub tests_failed: usize,
    pub warnings: usize,
    pub passed: bool,
    pub details: Vec<TestDetail>,
}

pub enum TestCategory {
    BuildValidation,
    ComponentRendering,
    StateManagement,
    AsyncOperations,
    ErrorBoundaries,
    EdgeCases,
    Performance,
    Accessibility,
}

pub struct TestDetail {
    pub name: String,
    pub passed: bool,
    pub duration: Duration,
    pub error: Option<String>,
}

pub struct ConfidenceBreakdown {
    pub overall: f32,
    pub build: f32,
    pub runtime: f32,
    pub error_handling: f32,
    pub state_management: f32,
    pub edge_cases: f32,
}
```

---

## Agent Communication

### Input from Quality Agent

```rust
pub struct QualityOutput {
    pub validated_code: HashMap<String, String>,
    pub quality_score: f32,
    pub issues_found: Vec<QualityIssue>,
    pub suggestions: Vec<String>,
    pub confidence: f32,
}
```

### Output to Project Initializer

```rust
pub struct DebugOutput {
    pub ready_for_deployment: bool,
    pub final_code: HashMap<String, String>,
    pub debug_report: DebugReport,
    pub confidence: f32,
    pub critical_issues: Vec<Bug>,
    pub warnings: Vec<Warning>,
    pub execution_summary: String,
}
```

---

## Testing Implementation

### Test Runner Architecture

```rust
pub struct TestRunner {
    sandbox: CodeSandbox,
    test_queue: VecDeque<Test>,
    results: Vec<TestResult>,
    fixes: Vec<AppliedFix>,
}

impl TestRunner {
    pub async fn run_all_tests(&mut self) -> Result<DebugReport> {
        // 1. Build validation
        self.run_build_tests().await?;
        
        // 2. Component tests
        self.run_component_tests().await?;
        
        // 3. State management tests
        self.run_state_tests().await?;
        
        // 4. Async tests
        self.run_async_tests().await?;
        
        // 5. Error boundary tests
        self.run_error_boundary_tests().await?;
        
        // 6. Edge case tests
        self.run_edge_case_tests().await?;
        
        // 7. Performance checks
        self.run_performance_tests().await?;
        
        // 8. Accessibility checks
        self.run_a11y_tests().await?;
        
        // Generate report
        self.generate_report()
    }
    
    async fn run_component_tests(&mut self) -> Result<()> {
        let components = self.sandbox.discover_components()?;
        
        for component in components {
            // Test 1: Basic render
            self.test_component_render(&component, None).await?;
            
            // Test 2: With valid props
            let props = self.generate_valid_props(&component)?;
            self.test_component_render(&component, Some(props)).await?;
            
            // Test 3: Edge cases
            self.test_component_edge_cases(&component).await?;
        }
        
        Ok(())
    }
    
    async fn test_component_render(
        &mut self,
        component: &Component,
        props: Option<Props>,
    ) -> Result<TestResult> {
        let result = self.sandbox.render(component, props).await;
        
        match result {
            Ok(_) => Ok(TestResult::Passed),
            Err(e) => {
                if self.is_auto_fixable(&e) {
                    let fix = self.generate_fix(&e)?;
                    self.fixes.push(fix);
                    Ok(TestResult::FixableFailure)
                } else {
                    Ok(TestResult::CriticalFailure(e))
                }
            }
        }
    }
}
```

### Code Sandbox

```rust
pub struct CodeSandbox {
    runtime: V8Runtime,
    filesystem: VirtualFS,
    network: MockNetwork,
    console: ConsoleCapture,
}

impl CodeSandbox {
    pub async fn render(
        &mut self,
        component: &Component,
        props: Option<Props>,
    ) -> Result<RenderResult> {
        // Set up virtual DOM
        self.runtime.setup_dom()?;
        
        // Load component
        let component_code = self.filesystem.read(&component.path)?;
        self.runtime.evaluate(&component_code)?;
        
        // Attempt render
        let render_script = format!(
            "React.createElement({}, {})",
            component.name,
            props.map(|p| p.to_json()).unwrap_or("null".to_string())
        );
        
        let result = self.runtime.evaluate(&render_script)?;
        
        Ok(RenderResult {
            success: true,
            html: result.to_html()?,
            console_output: self.console.drain(),
            errors: vec![],
        })
    }
    
    pub fn discover_components(&self) -> Result<Vec<Component>> {
        let mut components = Vec::new();
        
        for file in self.filesystem.list_files()? {
            if file.ends_with(".tsx") || file.ends_with(".jsx") {
                let content = self.filesystem.read(&file)?;
                if self.is_component(&content) {
                    components.push(Component {
                        name: self.extract_component_name(&content)?,
                        path: file,
                        props: self.extract_prop_types(&content)?,
                    });
                }
            }
        }
        
        Ok(components)
    }
}
```

---

## Bug Detection Patterns

### Pattern Matching

```rust
pub struct BugDetector {
    patterns: Vec<BugPattern>,
}

pub struct BugPattern {
    pub name: String,
    pub regex: Regex,
    pub severity: BugSeverity,
    pub category: BugCategory,
    pub auto_fixable: bool,
    pub fix_template: Option<String>,
}

impl BugDetector {
    pub fn detect_bugs(&self, code: &str) -> Vec<Bug> {
        let mut bugs = Vec::new();
        
        for pattern in &self.patterns {
            if let Some(matches) = pattern.regex.find_iter(code) {
                for m in matches {
                    bugs.push(Bug {
                        id: format!("{}_{}", pattern.name, m.start()),
                        severity: pattern.severity.clone(),
                        category: pattern.category.clone(),
                        file: self.current_file.clone(),
                        line: self.get_line_number(code, m.start()),
                        description: pattern.name.clone(),
                        code_context: self.extract_context(code, m.start()),
                        auto_fixable: pattern.auto_fixable,
                        // ... other fields
                    });
                }
            }
        }
        
        bugs
    }
    
    fn load_patterns() -> Vec<BugPattern> {
        vec![
            BugPattern {
                name: "Null Pointer Access".to_string(),
                regex: Regex::new(r"\w+\.\w+(?!\?)").unwrap(),
                severity: BugSeverity::High,
                category: BugCategory::NullPointer,
                auto_fixable: true,
                fix_template: Some("$1?.$2 ?? defaultValue".to_string()),
            },
            BugPattern {
                name: "Array Mutation".to_string(),
                regex: Regex::new(r"\.push\(|\.splice\(|\.sort\(\)").unwrap(),
                severity: BugSeverity::Medium,
                category: BugCategory::StateMutation,
                auto_fixable: true,
                fix_template: Some("[...$array, $item]".to_string()),
            },
            BugPattern {
                name: "Unhandled Promise".to_string(),
                regex: Regex::new(r"\.then\([^)]+\)(?!\.catch)").unwrap(),
                severity: BugSeverity::High,
                category: BugCategory::UnhandledError,
                auto_fixable: true,
                fix_template: Some("$1.catch(handleError)".to_string()),
            },
            // ... more patterns
        ]
    }
}
```

---

## Metrics & Analytics

### Debug Metrics

```rust
pub struct DebugMetrics {
    pub total_execution_time: Duration,
    pub build_time: Duration,
    pub test_time: Duration,
    pub fix_time: Duration,
    pub validation_time: Duration,
    
    pub tests_run: usize,
    pub tests_passed: usize,
    pub tests_failed: usize,
    pub tests_skipped: usize,
    
    pub bugs_found: usize,
    pub bugs_fixed: usize,
    pub bugs_unfixed: usize,
    
    pub warnings_raised: usize,
    
    pub fix_attempts: usize,
    pub successful_fixes: usize,
    pub failed_fixes: usize,
    pub rollbacks: usize,
    
    pub confidence_initial: f32,
    pub confidence_final: f32,
    pub confidence_delta: f32,
}

impl DebugMetrics {
    pub fn to_summary(&self) -> String {
        format!(
            "Debug Metrics:
  Time: {:.1}s (build: {:.1}s, test: {:.1}s, fix: {:.1}s)
  Tests: {}/{} passed ({:.0}%)
  Bugs: {}/{} fixed ({:.0}%)
  Confidence: {:.0}% → {:.0}% ({:+.0}%)",
            self.total_execution_time.as_secs_f32(),
            self.build_time.as_secs_f32(),
            self.test_time.as_secs_f32(),
            self.fix_time.as_secs_f32(),
            self.tests_passed,
            self.tests_run,
            (self.tests_passed as f32 / self.tests_run as f32) * 100.0,
            self.bugs_fixed,
            self.bugs_found,
            (self.bugs_fixed as f32 / self.bugs_found as f32) * 100.0,
            self.confidence_initial,
            self.confidence_final,
            self.confidence_delta,
        )
    }
}
```

---

## Error Recovery

### Recovery Strategies

```rust
pub enum RecoveryStrategy {
    AutoFix,           // Try to fix automatically
    Regenerate,        // Ask Engineer to regenerate
    Skip,              // Skip test and continue
    Abort,             // Stop debugging
    ManualIntervention,// Request user help
}

impl DebugAgent {
    async fn handle_test_failure(
        &mut self,
        test: &Test,
        error: &TestError,
    ) -> Result<RecoveryStrategy> {
        match error.severity {
            BugSeverity::Critical => {
                if error.auto_fixable {
                    Ok(RecoveryStrategy::AutoFix)
                } else {
                    // Show error to user, ask what to do
                    Ok(RecoveryStrategy::ManualIntervention)
                }
            }
            BugSeverity::High => {
                if error.auto_fixable {
                    Ok(RecoveryStrategy::AutoFix)
                } else {
                    Ok(RecoveryStrategy::Regenerate)
                }
            }
            BugSeverity::Medium | BugSeverity::Low => {
                // Log warning and continue
                self.warnings.push(error.to_warning());
                Ok(RecoveryStrategy::Skip)
            }
        }
    }
    
    async fn apply_recovery(
        &mut self,
        strategy: RecoveryStrategy,
    ) -> Result<()> {
        match strategy {
            RecoveryStrategy::AutoFix => {
                self.transition_to_fixing().await?;
            }
            RecoveryStrategy::Regenerate => {
                self.request_regeneration().await?;
            }
            RecoveryStrategy::Skip => {
                // Continue to next test
            }
            RecoveryStrategy::Abort => {
                self.transition_to_error().await?;
            }
            RecoveryStrategy::ManualIntervention => {
                self.request_user_decision().await?;
            }
        }
        
        Ok(())
    }
}
```

---

## Best Practices

### DO:
- ✓ Test all critical paths
- ✓ Auto-fix safe bugs
- ✓ Be honest about limitations
- ✓ Provide clear error messages
- ✓ Show before/after for fixes
- ✓ Validate fixes thoroughly
- ✓ Track all changes
- ✓ Calculate accurate confidence
- ✓ Flag warnings clearly
- ✓ Give actionable suggestions

### DON'T:
- ✗ Auto-fix risky bugs
- ✗ Hide unfixable issues
- ✗ Make breaking changes silently
- ✗ Over-promise capabilities
- ✗ Skip critical tests
- ✗ Apply fixes without validation
- ✗ Lose original code
- ✗ Inflate confidence scores
- ✗ Ignore warnings
- ✗ Leave users without options

---

## Example Flows

### Happy Path: All Tests Pass

```
1. IDLE → ANALYZING (2s)
   - Load code
   - Identify components
   
2. ANALYZING → TESTING (15s)
   - Run all tests
   - All pass
   
3. TESTING → COMPLETE (instant)
   - Generate report
   - Confidence: 98%
   - Ready for deployment
```

### Fixable Bugs Path

```
1. IDLE → ANALYZING (2s)
   
2. ANALYZING → TESTING (12s)
   - Find 3 fixable bugs
   
3. TESTING → FIXING (5s)
   - Apply 3 fixes
   
4. FIXING → VALIDATING (4s)
   - Re-test all
   - All pass
   
5. VALIDATING → COMPLETE (instant)
   - Confidence: 91%
   - 3 bugs fixed
```

### Critical Bugs Path

```
1. IDLE → ANALYZING (2s)
   
2. ANALYZING → TESTING (8s)
   - Find 2 critical unfixable bugs
   
3. TESTING → ERROR (instant)
   - Show bugs
   - Request decision
   
4. ERROR → ANALYZING (if regenerate)
   OR
   ERROR → IDLE (if cancel)
```

---

## Integration Points

### With Quality Agent

```rust
// Receive validated code
let quality_output = quality_agent.complete().await?;
debug_agent.load_code(quality_output.validated_code)?;
```

### With Project Initializer

```rust
// Send final code and report
let debug_output = debug_agent.complete().await?;
if debug_output.ready_for_deployment {
    project_init.initialize(debug_output).await?;
}
```

### With User Interface

```rust
// State updates
debug_agent.on_state_change(|state| {
    ui.update_debug_card(state);
});

// User actions
ui.on_pause_requested(|| {
    debug_agent.pause();
});
```

---

## Performance Considerations

### Optimization Strategies

1. **Parallel Testing**: Run independent tests concurrently
2. **Incremental Testing**: Only re-test affected components after fixes
3. **Test Caching**: Cache test results for unchanged code
4. **Smart Test Ordering**: Run fast tests first, slow tests last
5. **Early Termination**: Stop on critical failures
6. **Lazy Loading**: Load test data on-demand

```rust
impl TestRunner {
    async fn run_parallel_tests(&mut self) -> Result<()> {
        let mut futures = vec![];
        
        for test in self.get_independent_tests() {
            futures.push(self.run_test(test));
        }
        
        let results = futures::future::join_all(futures).await;
        
        for result in results {
            self.process_result(result?)?;
        }
        
        Ok(())
    }
}
```

---

## Conclusion

The Debug Agent is the **final gatekeeper** before deployment. It ensures that the code doesn't just look good and pass static analysis - it actually **runs correctly** in a real environment.

**Key Principles:**
1. **Test ruthlessly** - Find bugs before users do
2. **Fix intelligently** - Auto-fix what's safe, flag what's risky
3. **Report honestly** - Be clear about what works and what doesn't
4. **Empower users** - Give them options, not dead ends

The Debug Agent transforms "compiled code" into "**production-ready code**" - and that's the difference between shipping fast and shipping right.