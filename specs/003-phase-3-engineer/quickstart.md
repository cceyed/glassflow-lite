# Quickstart: Engineer Agent System

**Feature**: 003-phase-3-engineer  
**Date**: 2025-10-01  
**Purpose**: End-to-end validation of Engineer Agent functionality

## Overview
This quickstart validates the complete Engineer Agent workflow: receiving an architecture plan, generating code files, performing quality checks, and producing compilable output with high confidence.

---

## Prerequisites

**Required**:
- Rust 1.75+ installed
- Node.js 18+ installed
- TypeScript 5.x installed
- Tauri CLI installed
- Sample architecture plan (provided below)

**Setup**:
```bash
# Install dependencies
cd /Users/carsonidsinga/glassflow-lite
npm install

# Build Tauri app
npm run tauri build

# Run tests
cargo test --manifest-path=src-tauri/Cargo.toml
npm run test
```

---

## Test Scenario: React Todo App (17 Files)

### Step 1: Prepare Sample Architecture Plan

**File**: `specs/003-phase-3-engineer/fixtures/sample-plan.json`

```json
{
  "projectName": "todo-app",
  "projectIntent": "WebApp",
  "techStack": {
    "language": "TypeScript",
    "runtime": "Node.js 18+",
    "framework": "React 18",
    "styling": "TailwindCSS",
    "stateManagement": "Zustand",
    "testing": "Vitest"
  },
  "fileStructure": {
    "directories": [
      { "path": "src", "purpose": "Source code" },
      { "path": "src/components", "purpose": "React components" },
      { "path": "src/store", "purpose": "State management" },
      { "path": "src/types", "purpose": "TypeScript types" },
      { "path": "src/styles", "purpose": "CSS styles" }
    ],
    "files": [
      {
        "path": "src/main.tsx",
        "purpose": "Application entry point",
        "estimatedLines": 25,
        "language": "TypeScript",
        "dependencies": ["react", "react-dom"]
      },
      {
        "path": "src/App.tsx",
        "purpose": "Root component",
        "estimatedLines": 70,
        "language": "TypeScript",
        "dependencies": ["react", "./components/TodoList", "./store/todoStore"]
      },
      {
        "path": "src/types/todo.ts",
        "purpose": "Todo type definitions",
        "estimatedLines": 15,
        "language": "TypeScript",
        "dependencies": []
      },
      {
        "path": "src/store/todoStore.ts",
        "purpose": "Zustand store for todos",
        "estimatedLines": 85,
        "language": "TypeScript",
        "dependencies": ["zustand", "./types/todo"]
      },
      {
        "path": "src/components/TodoList.tsx",
        "purpose": "Todo list component",
        "estimatedLines": 90,
        "language": "TypeScript",
        "dependencies": ["react", "./TodoItem", "../store/todoStore"]
      },
      {
        "path": "src/components/TodoItem.tsx",
        "purpose": "Individual todo item",
        "estimatedLines": 55,
        "language": "TypeScript",
        "dependencies": ["react", "../types/todo"]
      },
      {
        "path": "src/components/AddTodo.tsx",
        "purpose": "Add todo form",
        "estimatedLines": 45,
        "language": "TypeScript",
        "dependencies": ["react", "../store/todoStore"]
      },
      {
        "path": "src/components/TodoFilter.tsx",
        "purpose": "Filter controls",
        "estimatedLines": 40,
        "language": "TypeScript",
        "dependencies": ["react", "../store/todoStore"]
      },
      {
        "path": "src/components/TodoStats.tsx",
        "purpose": "Statistics display",
        "estimatedLines": 35,
        "language": "TypeScript",
        "dependencies": ["react", "../store/todoStore"]
      },
      {
        "path": "src/styles/index.css",
        "purpose": "Global styles",
        "estimatedLines": 50,
        "language": "CSS",
        "dependencies": []
      },
      {
        "path": "package.json",
        "purpose": "Package configuration",
        "estimatedLines": 30,
        "language": "JSON",
        "dependencies": []
      },
      {
        "path": "tsconfig.json",
        "purpose": "TypeScript configuration",
        "estimatedLines": 20,
        "language": "JSON",
        "dependencies": []
      },
      {
        "path": "vite.config.ts",
        "purpose": "Vite configuration",
        "estimatedLines": 15,
        "language": "TypeScript",
        "dependencies": ["vite"]
      },
      {
        "path": "index.html",
        "purpose": "HTML entry point",
        "estimatedLines": 15,
        "language": "HTML",
        "dependencies": []
      },
      {
        "path": "README.md",
        "purpose": "Project documentation",
        "estimatedLines": 40,
        "language": "Markdown",
        "dependencies": []
      },
      {
        "path": ".gitignore",
        "purpose": "Git ignore rules",
        "estimatedLines": 20,
        "language": "Text",
        "dependencies": []
      },
      {
        "path": "tailwind.config.js",
        "purpose": "Tailwind configuration",
        "estimatedLines": 15,
        "language": "JavaScript",
        "dependencies": []
      }
    ]
  },
  "dependencies": [
    { "name": "react", "version": "^18.2.0", "devOnly": false },
    { "name": "react-dom", "version": "^18.2.0", "devOnly": false },
    { "name": "zustand", "version": "^4.4.0", "devOnly": false },
    { "name": "vite", "version": "^5.0.0", "devOnly": true },
    { "name": "typescript", "version": "^5.0.0", "devOnly": true },
    { "name": "tailwindcss", "version": "^3.3.0", "devOnly": true },
    { "name": "vitest", "version": "^1.0.0", "devOnly": true }
  ]
}
```

---

### Step 2: Start Engineer Agent

**Command**:
```bash
# Run Tauri app in dev mode
npm run tauri dev
```

**UI Actions**:
1. Load sample architecture plan from `fixtures/sample-plan.json`
2. Configure output directory: `./output/todo-app`
3. Click "Start Generation" button

**Expected State Transitions**:
```
IDLE → ANALYZING_PLAN (2-3s) → GENERATING_CODE (30-60s) → REVIEWING (5-10s) → COMPLETE
```

---

### Step 3: Monitor Generation Progress

**Expected Events** (in order):
1. `engineer:state_changed` → `analyzing_plan`
2. `engineer:reasoning` → "Architecture plan contains 17 files"
3. `engineer:state_changed` → `generating_code`
4. `engineer:file_started` → `src/types/todo.ts` (1/17)
5. `engineer:file_completed` → `src/types/todo.ts` (15 lines, 100% confidence)
6. `engineer:progress` → 1/17 (5.9%)
7. ... (repeat for each file)
8. `engineer:state_changed` → `reviewing`
9. `engineer:quality_check_started`
10. `engineer:reasoning` → "Found 0 critical issues, 2 minor style issues"
11. `engineer:reasoning` → "Auto-fixed 2 style issues"
12. `engineer:quality_check_completed`
13. `engineer:state_changed` → `complete`
14. `engineer:generation_complete` → 17 files, ~540 lines, 95%+ confidence

**UI Validation**:
- Progress bar shows 0% → 100%
- Current file updates in real-time
- Reasoning panel shows generation steps
- LED indicator: ○ → ◐ → ● → ◑ → ✓

---

### Step 4: Validate Generated Code

**File System Check**:
```bash
cd output/todo-app

# Verify all files exist
ls -la src/
ls -la src/components/
ls -la src/store/
ls -la src/types/
ls -la src/styles/

# Count files (should be 17)
find . -type f | wc -l
```

**Expected Output**:
```
17 files generated
```

**TypeScript Compilation Check**:
```bash
cd output/todo-app

# Install dependencies
npm install

# Run TypeScript compiler
npx tsc --noEmit

# Expected: No errors
```

**Linting Check**:
```bash
# Run ESLint
npx eslint src/

# Expected: No errors (or only warnings)
```

---

### Step 5: Verify Quality Report

**Command**:
```typescript
const report = await invoke('get_quality_report');
console.log(report);
```

**Expected Output**:
```json
{
  "totalFiles": 17,
  "filesPassed": 17,
  "filesFailed": 0,
  "issuesFound": 2,
  "issuesFixed": 2,
  "criticalIssues": [],
  "fileReports": [
    {
      "path": "src/types/todo.ts",
      "passed": true,
      "issues": []
    },
    {
      "path": "src/components/TodoItem.tsx",
      "passed": true,
      "issues": []
    },
    // ... (all 17 files)
  ]
}
```

**Validation Criteria**:
- ✅ `filesPassed === totalFiles`
- ✅ `criticalIssues.length === 0`
- ✅ All files have valid syntax
- ✅ No 'any' types used
- ✅ All imports resolved

---

### Step 6: Verify Confidence Breakdown

**Command**:
```typescript
const confidence = await invoke('get_confidence_breakdown');
console.log(confidence);
```

**Expected Output**:
```json
{
  "overall": 95.3,
  "qualityScore": 97.1,
  "planAdherence": 100.0,
  "issuePenalty": 4.0
}
```

**Validation Criteria**:
- ✅ `overall >= 85.0` (high confidence threshold)
- ✅ `qualityScore >= 90.0`
- ✅ `planAdherence >= 95.0`
- ✅ `issuePenalty <= 20.0`

---

### Step 7: Run Generated Application

**Command**:
```bash
cd output/todo-app

# Start dev server
npm run dev

# Open browser to http://localhost:5173
```

**Manual Testing**:
1. ✅ App loads without errors
2. ✅ Can add new todo
3. ✅ Can toggle todo completion
4. ✅ Can delete todo
5. ✅ Filter controls work (All/Active/Completed)
6. ✅ Statistics update correctly
7. ✅ UI matches design (TailwindCSS styles applied)

---

## Edge Case Testing

### Test 1: Timeout Handling

**Setup**: Simulate slow LLM response (>2 minutes)

**Expected**:
1. `engineer:timeout_prompt` event emitted
2. UI shows timeout dialog with options: Continue / Cancel / Skip
3. User selects "Continue"
4. Generation resumes
5. File completes successfully

**Validation**:
```typescript
listen('engineer:timeout_prompt', async (event) => {
  assert(event.payload.elapsedSeconds >= 120);
  await invoke('handle_timeout_prompt', {
    filePath: event.payload.filePath,
    choice: 'continue',
  });
});
```

---

### Test 2: Error Recovery

**Setup**: Inject invalid architecture plan (missing dependencies)

**Expected**:
1. `engineer:state_changed` → `error`
2. `engineer:error` event with recoverable=true
3. UI shows error message with "Retry" button
4. User modifies plan (adds missing dependencies)
5. User clicks "Retry"
6. Generation succeeds

**Validation**:
```typescript
const result = await invoke('start_code_generation', { plan: invalidPlan });
assert(result.isErr());

const state = await invoke('get_engineer_state');
assert(state.type === 'error');
assert(state.recoverable === true);

await invoke('retry_generation', { modifiedPlan: validPlan });
// Should succeed
```

---

### Test 3: Concurrent Generation

**Setup**: Architecture plan with 10 independent files

**Expected**:
1. Multiple `engineer:file_started` events emitted simultaneously
2. `engineer:progress` shows `currentFiles` array with 3-5 files
3. Files complete in parallel (faster than sequential)
4. All files generated correctly

**Validation**:
```bash
# Time sequential vs concurrent generation
time npm run generate -- --sequential  # ~60s
time npm run generate -- --concurrent  # ~20s (3x faster)
```

---

### Test 4: Partial Generation Preservation

**Setup**: Cancel generation mid-way (after 5/17 files)

**Expected**:
1. User clicks "Cancel" button
2. `cancel_generation` command called
3. Successfully completed files (5) preserved in output directory
4. Partial/incomplete files (12) discarded
5. State transitions to IDLE

**Validation**:
```bash
cd output/todo-app
find . -type f | wc -l
# Expected: 5 (only completed files)
```

---

## Performance Benchmarks

**Target Metrics**:
- File generation: <2 minutes per file (with timeout)
- Total generation (17 files): <90 seconds
- Quality checks: <10 seconds for all files
- Confidence calculation: <1 second
- UI responsiveness: 60fps during generation

**Measurement**:
```typescript
const start = Date.now();
await invoke('start_code_generation', { plan, config });

listen('engineer:generation_complete', (event) => {
  const duration = Date.now() - start;
  console.log(`Total duration: ${duration}ms`);
  assert(duration < 90000); // <90 seconds
});
```

---

## Success Criteria

**Must Pass**:
- ✅ All 17 files generated
- ✅ TypeScript compilation passes (no errors)
- ✅ All quality checks pass
- ✅ Confidence >= 85%
- ✅ No critical issues
- ✅ Generated app runs successfully
- ✅ All state transitions valid
- ✅ All events emitted correctly
- ✅ Timeout handling works
- ✅ Error recovery works
- ✅ Concurrent generation works
- ✅ Partial preservation works

**Performance**:
- ✅ Total generation <90 seconds
- ✅ Per-file generation <2 minutes
- ✅ Quality checks <10 seconds
- ✅ UI remains responsive (60fps)

---

## Cleanup

```bash
# Remove generated output
rm -rf output/todo-app

# Reset Engineer Agent state
await invoke('reset_engineer_state');
```

---

**Status**: Quickstart complete. Ready for task generation (/tasks command).
