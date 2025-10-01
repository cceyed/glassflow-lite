# PHASE 4: QUALITY AGENT

## Overview

The Quality Agent is the critical code reviewer in the Glassflow pipeline. It receives code from the Engineer Agent and performs comprehensive quality checks, identifying bugs, security issues, code smells, and standards violations. It can auto-fix minor issues and reports critical problems that require attention.

**Position in Pipeline:** 3rd (after Architect → Engineer → **Quality**)

**Critical Role:** Gatekeeper - ensures only high-quality code proceeds to Debug stage

---

## Core Purpose & Philosophy

### The Quality Agent's Mission
To be the **uncompromising guardian of code quality**. It catches what Engineer might miss, enforces standards, and ensures the codebase is maintainable, secure, and performant.

### Key Characteristics
- **Thorough**: Checks everything, misses nothing critical
- **Strict but Fair**: High standards, but understands context
- **Helpful**: Doesn't just point out problems, suggests solutions
- **Fast**: Reviews in seconds, not minutes
- **Transparent**: Shows reasoning for every finding

---

## State Machine

### States & Transitions

```
┌─────────────────────────────────────────────────────┐
│                                                      │
│  IDLE → ANALYZING → REVIEWING → FIXING → COMPLETE  │
│           ↓           ↓           ↓                 │
│         ERROR ← ───── ┴ ────────  ┘                 │
│                                                      │
└─────────────────────────────────────────────────────┘
```

### IDLE State
**Visual:**
- LED: ○ (hollow circle, #404040)
- Status: "Ready"
- Panel: Collapsed

**Behavior:**
- Waiting for Engineer to complete
- No processing
- Minimal UI presence

**Transitions:**
- → ANALYZING when Engineer completes

---

### ANALYZING State
**Visual:**
- LED: ◐ (half-filled, pulsing white)
- Status: "Analyzing code structure..."
- Panel: Shows file count being loaded

**Behavior:**
- Parse all generated files
- Build dependency graph
- Identify critical paths
- Plan review strategy
- Load quality rules

**Duration:** 2-4 seconds

**Reasoning Output:**
```
[QUALITY] Loading generated code...
[QUALITY] → Found 17 files (847 lines)
[QUALITY] → Parsing TypeScript files
[QUALITY] → Building import graph
[QUALITY] → 12 components, 3 utilities, 2 configs
[QUALITY] → Planning review strategy
```

**Transitions:**
- → REVIEWING when analysis complete
- → ERROR if files cannot be parsed

---

### REVIEWING State
**Visual:**
- LED: ● (filled circle, bright white + glow)
- Status: "Reviewing [filename]..."
- Progress: ▓▓▓▓▓░░░░░░ 47% (8/17 files)
- Panel: Shows current checks being run

**Behavior:**
- Run comprehensive quality checks on each file:
  1. Syntax & Parsing
  2. Type Safety
  3. Import/Export Validation
  4. Code Smells
  5. Security Issues
  6. Performance Concerns
  7. Best Practices
  8. Standards Compliance
  9. Documentation Coverage
  10. Error Handling

- Real-time issue detection
- Categorize issues by severity
- Calculate confidence per file
- Emit progress updates

**Duration:** 5-15 seconds (depends on project size)

**Reasoning Output:**
```
[QUALITY] Reviewing src/App.tsx...
[QUALITY] → Syntax: ✓ Valid
[QUALITY] → Types: ✓ All explicit
[QUALITY] → Imports: ✓ Resolved
[QUALITY] → Security: ✓ No issues
[QUALITY] → Performance: ✓ Optimized
[QUALITY] → Confidence: 98%

[QUALITY] Reviewing src/components/TodoList.tsx...
[QUALITY] → Syntax: ✓ Valid
[QUALITY] → Types: ⚠ Implicit any on line 45
[QUALITY] → Imports: ✓ Resolved
[QUALITY] → Code Smell: ⚠ Long function (78 lines)
[QUALITY] → Performance: ⚠ Missing useMemo
[QUALITY] → Confidence: 82%
[QUALITY] → Found 3 fixable issues
```

**Transitions:**
- → FIXING if auto-fixable issues found
- → COMPLETE if no fixable issues
- → ERROR if critical blocking issues found

---

### FIXING State
**Visual:**
- LED: ◑ (inverted half, pulsing slower)
- Status: "Applying fixes..."
- Progress: Fix count (e.g., "Fixed 3/5 issues")
- Panel: Shows fixes being applied

**Behavior:**
- Auto-fix minor/medium severity issues:
  - Add missing types
  - Remove unused imports
  - Fix formatting
  - Add error handling
  - Optimize performance
  
- Re-validate after fixes
- Update confidence scores
- Log all changes

**Duration:** 2-5 seconds

**Reasoning Output:**
```
[QUALITY] Applying automatic fixes...

[QUALITY] → Fix 1/3: Adding type annotation
  File: src/components/TodoList.tsx:45
  Before: const handleClick = (id) => { ... }
  After:  const handleClick = (id: string) => { ... }
  ✓ Applied

[QUALITY] → Fix 2/3: Wrapping in useMemo
  File: src/components/TodoList.tsx:23
  Before: const filtered = todos.filter(...)
  After:  const filtered = useMemo(() => todos.filter(...), [todos])
  ✓ Applied

[QUALITY] → Fix 3/3: Removing unused import
  File: src/App.tsx:3
  Removed: import { useState } from 'react'
  ✓ Applied

[QUALITY] Re-validating fixed files...
[QUALITY] → All fixes successful
[QUALITY] → Updated confidence: 82% → 94%
```

**Transitions:**
- → REVIEWING to re-check fixed files
- → COMPLETE if all fixes applied successfully
- → ERROR if fixes cause new issues

---

### COMPLETE State
**Visual:**
- LED: ✓ (checkmark, white with subtle green glow)
- Status: "Review complete"
- Confidence: XX%
- Panel: Shows summary of findings

**Behavior:**
- Generate quality report
- Calculate final confidence
- Prepare output for Debug agent
- Display summary to user
- Auto-transition to IDLE after handoff

**Duration:** Instant display, 2s before auto-collapse

**Reasoning Output:**
```
[QUALITY] Review complete!

Summary:
• Files reviewed: 17
• Issues found: 8 (5 fixed, 3 warnings)
• Critical: 0
• High: 0
• Medium: 3 (auto-fixed)
• Low: 5 (2 fixed, 3 warnings)

Confidence: 92%
├─ Code Quality: 95%
├─ Type Safety: 98%
├─ Security: 100%
├─ Performance: 88%
└─ Maintainability: 90%

Time: 12.3s
Ready for Debug stage ✓
```

**Transitions:**
- → IDLE after 2 seconds
- Passes QualityReport to Debug Agent

---

### ERROR State
**Visual:**
- LED: ✗ (X mark, white with red glow tint)
- Status: "Critical issues found"
- Panel: Shows error details and options

**Behavior:**
- Display critical blocking issues
- Explain why they're blocking
- Suggest remediation steps
- Offer user options:
  1. Return to Engineer (modify code)
  2. Manual review/fix
  3. Override (dangerous, requires confirmation)
  4. Cancel build

**Reasoning Output:**
```
[QUALITY] ✗ Critical issues found - cannot proceed

Critical Issues:
1. Type Safety Violation (BLOCKING)
   File: src/components/TodoItem.tsx:23
   Issue: Unsafe type assertion
   Risk: Runtime error, potential crash
   Fix: Add proper type guard
   
2. Security Issue (BLOCKING)
   File: src/api/client.ts:15
   Issue: XSS vulnerability in user input
   Risk: Code injection attack
   Fix: Sanitize input before rendering

These issues must be resolved before proceeding.

Options:
[Fix Automatically] [Return to Engineer] [Manual Review] [Cancel]
```

**Transitions:**
- → IDLE if user cancels
- → REVIEWING if automatic fix attempted
- Stays in ERROR until user takes action

---

## Quality Checks System

### Check Categories

#### 1. Syntax & Parsing
**Purpose:** Ensure code is syntactically valid

**Checks:**
- Valid TypeScript/JavaScript syntax
- No parsing errors
- Proper file encoding (UTF-8)
- No syntax ambiguities

**Example Issues:**
```
✗ Syntax Error
  File: src/App.tsx:45
  Issue: Unexpected token '}'
  Context: Missing closing parenthesis on line 44
  Severity: CRITICAL
```

---

#### 2. Type Safety
**Purpose:** Enforce TypeScript type correctness

**Checks:**
- No `any` types (unless explicitly allowed)
- All function parameters typed
- All return types specified
- No unsafe type assertions
- Proper generic usage
- No type errors

**Example Issues:**
```
⚠ Type Safety
  File: src/components/TodoList.tsx:23
  Issue: Parameter 'id' implicitly has 'any' type
  Suggestion: Add explicit type: (id: string) => {...}
  Severity: MEDIUM
  Auto-fixable: Yes

✗ Type Safety
  File: src/utils/api.ts:12
  Issue: Unsafe type assertion (response as User)
  Risk: Runtime error if response structure changes
  Suggestion: Add type guard or validation
  Severity: HIGH
  Auto-fixable: No
```

---

#### 3. Import/Export Validation
**Purpose:** Ensure module system integrity

**Checks:**
- All imports resolve
- No circular dependencies
- No unused imports
- Exports match usage
- Proper module structure
- No duplicate imports

**Example Issues:**
```
⚠ Import Issue
  File: src/App.tsx:3
  Issue: Unused import 'useState'
  Suggestion: Remove unused import
  Severity: LOW
  Auto-fixable: Yes

✗ Import Issue
  File: src/components/TodoList.tsx:2
  Issue: Circular dependency detected
  Path: TodoList → TodoItem → TodoList
  Risk: Bundle errors, runtime issues
  Severity: CRITICAL
  Auto-fixable: No
```

---

#### 4. Code Smells
**Purpose:** Identify maintainability issues

**Checks:**
- Function length (>50 lines = warning, >100 = error)
- Cyclomatic complexity
- Deep nesting (>4 levels)
- Duplicate code
- Magic numbers
- Long parameter lists
- God objects/classes

**Example Issues:**
```
⚠ Code Smell
  File: src/components/TodoList.tsx:15-92
  Issue: Function too long (78 lines)
  Suggestion: Extract into smaller functions
  Severity: MEDIUM
  Auto-fixable: No

⚠ Code Smell
  File: src/utils/helpers.ts:23
  Issue: Magic number '86400000'
  Suggestion: Extract to named constant (MS_PER_DAY)
  Severity: LOW
  Auto-fixable: Yes
```

---

#### 5. Security Issues
**Purpose:** Catch security vulnerabilities

**Checks:**
- XSS vulnerabilities
- SQL injection risks
- Command injection
- Path traversal
- Insecure randomness
- Exposed secrets
- Unsafe eval/Function usage
- CORS misconfigurations

**Example Issues:**
```
✗ Security Issue
  File: src/components/Display.tsx:34
  Issue: Potential XSS - dangerouslySetInnerHTML
  Risk: User input rendered without sanitization
  Suggestion: Use textContent or sanitize with DOMPurify
  Severity: CRITICAL
  Auto-fixable: No

⚠ Security Issue
  File: src/config/api.ts:5
  Issue: API key hardcoded in source
  Suggestion: Move to environment variable
  Severity: HIGH
  Auto-fixable: No
```

---

#### 6. Performance Concerns
**Purpose:** Identify performance issues

**Checks:**
- Missing memoization (React)
- Inefficient loops
- N+1 query patterns
- Large bundle sizes
- Blocking operations
- Memory leaks
- Unnecessary re-renders

**Example Issues:**
```
⚠ Performance
  File: src/components/TodoList.tsx:23
  Issue: Expensive computation without memoization
  Impact: Re-computes on every render
  Suggestion: Wrap in useMemo
  Severity: MEDIUM
  Auto-fixable: Yes

⚠ Performance
  File: src/utils/sort.ts:12
  Issue: Nested loop with O(n²) complexity
  Suggestion: Use Map for O(n) lookup
  Severity: MEDIUM
  Auto-fixable: No
```

---

#### 7. Best Practices
**Purpose:** Enforce coding standards

**Checks:**
- Proper naming conventions
- Consistent formatting
- Appropriate comments
- Error handling patterns
- Async/await usage
- React hooks rules
- Component patterns

**Example Issues:**
```
⚠ Best Practice
  File: src/components/todoitem.tsx
  Issue: Component filename should be PascalCase
  Suggestion: Rename to TodoItem.tsx
  Severity: LOW
  Auto-fixable: Yes

⚠ Best Practice
  File: src/hooks/useTodos.ts:12
  Issue: Async function without error handling
  Suggestion: Add try-catch or .catch()
  Severity: MEDIUM
  Auto-fixable: Yes
```

---

#### 8. Standards Compliance
**Purpose:** Match user-defined standards

**Checks:**
- Indent style (spaces/tabs)
- Line length limits
- Semicolon usage
- Quote style (single/double)
- Trailing commas
- Import ordering
- File organization

**Example Issues:**
```
⚠ Standards
  File: src/App.tsx:23
  Issue: Line exceeds 80 character limit (94 chars)
  Suggestion: Break into multiple lines
  Severity: LOW
  Auto-fixable: Yes

⚠ Standards
  File: src/types/todo.ts:5
  Issue: Missing trailing comma
  Suggestion: Add trailing comma per standards
  Severity: LOW
  Auto-fixable: Yes
```

---

#### 9. Documentation Coverage
**Purpose:** Ensure code is well-documented

**Checks:**
- JSDoc comments on public functions
- Complex logic explained
- Type documentation
- README completeness
- Inline comments for non-obvious code

**Example Issues:**
```
⚠ Documentation
  File: src/utils/validation.ts:12
  Issue: Public function lacks JSDoc comment
  Suggestion: Add description, params, returns
  Severity: LOW
  Auto-fixable: No (requires human input)

⚠ Documentation
  File: src/components/TodoList.tsx:45
  Issue: Complex algorithm without explanation
  Suggestion: Add comment explaining filtering logic
  Severity: LOW
  Auto-fixable: No
```

---

#### 10. Error Handling
**Purpose:** Ensure robust error handling

**Checks:**
- Try-catch around async operations
- Promise rejection handling
- Null/undefined checks
- Optional chaining usage
- Error boundaries (React)
- Proper error messages

**Example Issues:**
```
⚠ Error Handling
  File: src/api/client.ts:23
  Issue: Unhandled promise rejection
  Risk: Silent failure, poor UX
  Suggestion: Add .catch() or try-catch
  Severity: HIGH
  Auto-fixable: Yes

⚠ Error Handling
  File: src/components/TodoItem.tsx:34
  Issue: Potential null access without check
  Suggestion: Use optional chaining (todo?.title)
  Severity: MEDIUM
  Auto-fixable: Yes
```

---

## Issue Severity System

### CRITICAL (Blocking)
**Criteria:**
- Prevents code from running
- Security vulnerability
- Data loss risk
- System crash potential

**Behavior:**
- Stops pipeline immediately
- Requires fix before proceeding
- Cannot be auto-fixed
- User must take action

**Examples:**
- Type errors that prevent compilation
- XSS vulnerabilities
- Circular dependencies
- SQL injection risks

---

### HIGH (Major Issue)
**Criteria:**
- Significant bug likely
- Poor performance impact
- Maintainability severely affected
- Security concern (not exploitable yet)

**Behavior:**
- Flags for review
- Some auto-fixable
- Lowers confidence significantly
- Recommended to fix

**Examples:**
- Unsafe type assertions
- Missing error handling on critical paths
- N+1 query patterns
- Exposed API keys

---

### MEDIUM (Should Fix)
**Criteria:**
- Code smell
- Minor performance issue
- Maintainability concern
- Standards violation

**Behavior:**
- Often auto-fixable
- Reported in summary
- Moderate confidence impact
- Can proceed if confidence threshold met

**Examples:**
- Missing memoization
- Long functions
- Implicit any types
- Missing type annotations

---

### LOW (Nice to Fix)
**Criteria:**
- Cosmetic issue
- Minor style violation
- Documentation gap
- Unused code

**Behavior:**
- Always auto-fixable or ignorable
- Minimal confidence impact
- Listed in full report
- Optional fixes

**Examples:**
- Unused imports
- Formatting inconsistencies
- Missing trailing commas
- Minor naming issues

---

## Auto-Fix System

### Auto-Fixable Issues

**Type Annotations:**
```typescript
// Before
const handleClick = (id) => { ... }

// After
const handleClick = (id: string) => { ... }
```

**Unused Imports:**
```typescript
// Before
import { useState, useEffect } from 'react'; // useState unused

// After
import { useEffect } from 'react';
```

**Missing Memoization:**
```typescript
// Before
const filtered = todos.filter(t => t.completed);

// After
const filtered = useMemo(
  () => todos.filter(t => t.completed),
  [todos]
);
```

**Error Handling:**
```typescript
// Before
const data = await fetchData();

// After
try {
  const data = await fetchData();
} catch (error) {
  console.error('Failed to fetch data:', error);
  // Handle error appropriately
}
```

**Optional Chaining:**
```typescript
// Before
const title = todo.title;

// After
const title = todo?.title;
```

**Formatting:**
```typescript
// Before
const obj={a:1,b:2}

// After
const obj = { a: 1, b: 2 };
```

### Auto-Fix Strategy

**Safety First:**
1. Only fix issues with 100% certainty
2. Never change logic/behavior
3. Preserve functionality
4. Re-validate after fixes

**Fix Order:**
1. Syntax/formatting (safest)
2. Unused code removal
3. Type annotations
4. Performance optimizations
5. Error handling (most complex)

**Limitations:**
- Cannot fix logic errors
- Cannot add missing features
- Cannot resolve design issues
- Cannot fix security vulnerabilities (too risky)

---

## Confidence Calculation

### Formula

```rust
fn calculate_quality_confidence(report: &QualityReport) -> ConfidenceBreakdown {
    // Individual category scores (0-1)
    let code_quality = calculate_code_quality_score(report);
    let type_safety = calculate_type_safety_score(report);
    let security = calculate_security_score(report);
    let performance = calculate_performance_score(report);
    let maintainability = calculate_maintainability_score(report);
    
    // Issue penalty
    let issue_penalty = calculate_issue_penalty(&report.issues);
    
    // Weighted average
    let overall = (
        code_quality * 0.25 +
        type_safety * 0.25 +
        security * 0.25 +
        performance * 0.15 +
        maintainability * 0.10
    ) * (1.0 - issue_penalty);
    
    ConfidenceBreakdown {
        overall: (overall * 100.0).clamp(0.0, 100.0),
        code_quality: (code_quality * 100.0),
        type_safety: (type_safety * 100.0),
        security: (security * 100.0),
        performance: (performance * 100.0),
        maintainability: (maintainability * 100.0),
    }
}

fn calculate_issue_penalty(issues: &[Issue]) -> f32 {
    let mut penalty = 0.0;
    
    for issue in issues {
        penalty += match issue.severity {
            Severity::Critical => 0.30, // 30% penalty per critical
            Severity::High => 0.15,     // 15% penalty per high
            Severity::Medium => 0.05,   // 5% penalty per medium
            Severity::Low => 0.01,      // 1% penalty per low
        };
    }
    
    penalty.min(0.80) // Cap at 80% total penalty
}
```

### Confidence Thresholds

- **95-100%**: Excellent, production-ready
- **85-94%**: Good, minor improvements possible
- **70-84%**: Acceptable, some concerns
- **50-69%**: Poor, significant issues
- **0-49%**: Critical issues, cannot proceed

---

## UI Components

### Quality Card (Collapsed)

```
┌────────────────────────────────────────┐
│ ● QUALITY                         92%  │
│ Reviewing src/components/Todo.tsx      │
│ ▓▓▓▓▓▓▓▓▓▓▓░░░░ 73% (12/17 files)    │
└────────────────────────────────────────┘
```

### Quality Panel (Expanded - Reviewing)

```
┌─────────────────────────────────────────────────────┐
│ [●] QUALITY                        Confidence: 92%  │
│ ─────────────────────────────────────────────────── │
│                                                      │
│ Phase: Reviewing Code                               │
│ Current File: src/components/TodoList.tsx           │
│ Progress: 12/17 files (71%)                         │
│ Elapsed: 8.7s | Estimated: 3.1s remaining           │
│                                                      │
│ Checks Running:                                     │
│ ✓ Syntax & Parsing                                  │
│ ✓ Type Safety                                       │
│ ✓ Imports/Exports                                   │
│ ● Security (in progress...)                         │
│ ○ Performance                                       │
│ ○ Best Practices                                    │
│ ○ Standards                                         │
│                                                      │
│ Issues Found So Far:                                │
│ • Critical: 0                                       │
│ • High: 0                                           │
│ • Medium: 3 (2 auto-fixed)                          │
│ • Low: 7 (5 auto-fixed)                             │
│                                                      │
│ Files Reviewed:                                     │
│ ✓ src/App.tsx (98% confidence)                     │
│ ✓ src/main.tsx (99% confidence)                    │
│ ✓ src/store/todoStore.ts (96% confidence)          │
│ ⚠ src/components/TodoList.tsx (82% - 3 issues)     │
│ ● src/components/TodoItem.tsx (reviewing...)        │
│                                                      │
│ Reasoning:                                          │
│ ○ High code quality overall                         │
│ ○ Type safety excellent (no any types)              │
│ ○ Some performance optimizations needed             │
│ ○ Minor formatting issues (auto-fixing)             │
│                                                      │
│ ─────────────────────────────────────────────────── │
│ [Pause Review] [View Issues] [Skip File]           │
└─────────────────────────────────────────────────────┘
```

### Quality Panel (Complete - Summary)

```
┌─────────────────────────────────────────────────────┐
│ [✓] QUALITY                        Confidence: 92%  │
│ ─────────────────────────────────────────────────── │
│                                                      │
│ Review Complete                         Time: 12.3s │
│                                                      │
│ Summary:                                            │
│ • Files Reviewed: 17                                │
│ • Total Lines: 847                                  │
│ • Issues Found: 10                                  │
│   ├─ Critical: 0 ✓                                  │
│   ├─ High: 0 ✓                                      │
│   ├─ Medium: 3 (all fixed) ✓                        │
│   └─ Low: 7 (5 fixed, 2 warnings)                  │
│                                                      │
│ Confidence Breakdown:                               │
│ ├─ Code Quality:     ████████████░ 95%             │
│ ├─ Type Safety:      ██████████████ 98%            │
│ ├─ Security:         ███████████████ 100%          │
│ ├─ Performance:      ████████░░░ 88%               │
│ └─ Maintainability:  █████████░░ 90%               │
│                                                      │
│ Fixes Applied: 8                                    │
│ ✓ Added 3 type annotations                          │
│ ✓ Removed 5 unused imports                          │
│ ✓ Applied 2 performance optimizations               │
│                                                      │
│ Remaining Warnings: 2 (non-blocking)                │
│ ⚠ Long function in TodoList.tsx (78 lines)         │
│ ⚠ Missing JSDoc in validation.ts                    │
│                                                      │
│ ─────────────────────────────────────────────────── │
│ [View Full Report] [Export] [Continue to Debug]    │
└─────────────────────────────────────────────────────┘
```

### Issue Detail View

```
┌─────────────────────────────────────────────────────┐
│ Issue Details                                       │
├─────────────────────────────────────────────────────┤
│                                                      │
│ ⚠ Medium Severity - Auto-Fixed                      │
│                                                      │
│ Type Safety Issue                                   │
│ File: src/components/TodoList.tsx                   │
│ Line: 45                                            │
│                                                      │
│ Description:                                        │
│ Parameter 'id' implicitly has 'any' type. This     │
│ reduces type safety and can lead to runtime errors. │
│                                                      │
│ Impact:                                             │
│ • Loss of type checking                             │
│ • Potential runtime errors                          │
│ • Reduced IDE autocomplete                          │
│                                                      │
│ Code:                                               │
│ ─────────────────────────────────────────────────── │
│  43  const TodoList = () => {                       │
│  44    const handleClick = (id) => {                │
│  45      deleteTodo(id);                            │
│  46    };                                           │
│ ─────────────────────────────────────────────────── │
│                                                      │
│ Suggested Fix:                                      │
│ Add explicit type annotation for parameter          │
│                                                      │
│ Fixed Code:                                         │
│ ─────────────────────────────────────────────────── │
│  43  const TodoList = () => {                       │
│  44    const handleClick = (id: string) => {        │
│  45      deleteTodo(id);                            │
│  46    };                                           │
│ ─────────────────────────────────────────────────── │
│                                                      │
│ ✓ Fix Applied Successfully                          │
│ Updated Confidence: 82% → 85%                       │
│                                                      │
│ ─────────────────────────────────────────────────── │
│ [Next Issue] [View File] [Close]                   │
└─────────────────────────────────────────────────────┘
```

---

Backend Implementation
Rust Service Structure
rust// src-tauri/src/agents/quality.rs

pub struct QualityAgent {
    llm_client: Arc<LLMClient>,
    checker: QualityChecker,
    fixer: AutoFixer,
    config: QualityConfig,
    state: QualityState,
}

pub struct QualityConfig {
    pub strictness: StrictnessLevel,
    pub auto_fix_enabled: bool,
    pub max_auto_fixes: usize,
    pub confidence_threshold: f32,
    pub check_categories: Vec<CheckCategory>,
}

pub enum StrictnessLevel {
    Relaxed,   // Fewer checks, more lenient
    Standard,  // Balanced approach
    Strict,    // Maximum checks, zero tolerance
}

pub enum QualityState {
    Idle,
    Analyzing {
        files: Vec<GeneratedFile>,
    },
    Reviewing {
        files: Vec<GeneratedFile>,
        current_index: usize,
        issues_found: Vec<Issue>,
        progress: f32,
    },
    Fixing {
        files: Vec<GeneratedFile>,
        fixes_applied: Vec<AppliedFix>,
        fixes_remaining: usize,
    },
    Complete {
        report: QualityReport,
        duration: Duration,
    },
    Error {
        message: String,
        blocking_issues: Vec<Issue>,
    },
}

pub struct QualityReport {
    pub files_reviewed: usize,
    pub total_lines: usize,
    pub issues: Vec<Issue>,
    pub fixes_applied: Vec<AppliedFix>,
    pub confidence: ConfidenceBreakdown,
    pub checks_performed: Vec<CheckResult>,
}

pub struct Issue {
    pub id: String,
    pub severity: Severity,
    pub category: CheckCategory,
    pub file: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub description: String,
    pub impact: String,
    pub suggestion: String,
    pub auto_fixable: bool,
    pub code_context: Option<CodeContext>,
}

pub struct AppliedFix {
    pub issue_id: String,
    pub file: String,
    pub description: String,
    pub before: String,
    pub after: String,
    pub confidence: f32,
}

impl QualityAgent {
    pub async fn review_code(&mut self, engineer_output: CodeOutput) -> Result<QualityReport> {
        let start_time = Instant::now();
        
        // Analyzing
        self.state = QualityState::Analyzing {
            files: engineer_output.files.clone(),
        };
        
        self.emit_reasoning(&format!(
            "Analyzing {} files ({} lines)",
            engineer_output.files.len(),
            engineer_output.total_lines
        ));
        
        // Parse and prepare
        let parsed_files = self.parse_files(&engineer_output.files).await?;
        
        // Reviewing
        self.state = QualityState::Reviewing {
            files: parsed_files.clone(),
            current_index: 0,
            issues_found: Vec::new(),
            progress: 0.0,
        };
        
        let mut all_issues = Vec::new();
        let total_files = parsed_files.len();
        
        // Review each file
        for (index, file) in parsed_files.iter().enumerate() {
            self.emit_reasoning(&format!(
                "Reviewing {} ({}/{})",
                file.path, index + 1, total_files
            ));
            
            if let QualityState::Reviewing { current_index, .. } = &mut self.state {
                *current_index = index;
            }
            
            // Run all quality checks
            let file_issues = self.check_file(file).await?;
            
            if !file_issues.is_empty() {
                self.emit_reasoning(&format!(
                    "Found {} issues in {}",
                    file_issues.len(),
                    file.path
                ));
            }
            
            all_issues.extend(file_issues);
            
            let progress = (index + 1) as f32 / total_files as f32;
            self.emit_progress(progress);
        }
        
        self.emit_reasoning(&format!(
            "Review complete: {} issues found",
            all_issues.len()
        ));
        
        // Auto-fix if enabled
        let mut fixes_applied = Vec::new();
        if self.config.auto_fix_enabled && !all_issues.is_empty() {
            self.state = QualityState::Fixing {
                files: parsed_files.clone(),
                fixes_applied: Vec::new(),
                fixes_remaining: all_issues.iter()
                    .filter(|i| i.auto_fixable)
                    .count(),
            };
            
            fixes_applied = self.auto_fix_issues(&mut parsed_files, &all_issues).await?;
            
            if !fixes_applied.is_empty() {
                self.emit_reasoning(&format!(
                    "Auto-fixed {} issues",
                    fixes_applied.len()
                ));
                
                // Re-validate fixed files
                all_issues = self.revalidate_files(&parsed_files).await?;
            }
        }
        
        // Generate report
        let confidence = self.calculate_confidence(&all_issues, &parsed_files);
        
        let checks_performed = self.get_check_results();
        
        let report = QualityReport {
            files_reviewed: total_files,
            total_lines: parsed_files.iter().map(|f| f.lines).sum(),
            issues: all_issues.clone(),
            fixes_applied,
            confidence,
            checks_performed,
        };
        
        let duration = start_time.elapsed();
        
        // Check for blocking issues
        let blocking_issues: Vec<_> = all_issues.iter()
            .filter(|i| matches!(i.severity, Severity::Critical))
            .cloned()
            .collect();
        
        if !blocking_issues.is_empty() {
            self.state = QualityState::Error {
                message: format!("{} critical issues found", blocking_issues.len()),
                blocking_issues,
            };
            
            self.emit_error(&report);
            return Err(anyhow::anyhow!("Critical issues found"));
        }
        
        // Complete
        self.state = QualityState::Complete {
            report: report.clone(),
            duration,
        };
        
        self.emit_reasoning(&format!(
            "Quality review complete ({}s, {}% confidence)",
            duration.as_secs(),
            confidence.overall
        ));
        
        self.emit_complete(&report);
        
        Ok(report)
    }
    
    async fn check_file(&self, file: &ParsedFile) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        
        // Run all enabled checks
        for category in &self.config.check_categories {
            let check_issues = match category {
                CheckCategory::Syntax => self.check_syntax(file),
                CheckCategory::TypeSafety => self.check_type_safety(file),
                CheckCategory::Imports => self.check_imports(file),
                CheckCategory::CodeSmells => self.check_code_smells(file),
                CheckCategory::Security => self.check_security(file),
                CheckCategory::Performance => self.check_performance(file),
                CheckCategory::BestPractices => self.check_best_practices(file),
                CheckCategory::Standards => self.check_standards(file),
                CheckCategory::Documentation => self.check_documentation(file),
                CheckCategory::ErrorHandling => self.check_error_handling(file),
            }?;
            
            issues.extend(check_issues);
        }
        
        Ok(issues)
    }
    
    fn check_type_safety(&self, file: &ParsedFile) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        
        // Check for 'any' types
        for any_usage in &file.any_type_usages {
            issues.push(Issue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::Medium,
                category: CheckCategory::TypeSafety,
                file: file.path.clone(),
                line: Some(any_usage.line),
                column: Some(any_usage.column),
                description: "Using 'any' type loses type safety".to_string(),
                impact: "Reduced type checking, potential runtime errors".to_string(),
                suggestion: "Use specific type or generic instead".to_string(),
                auto_fixable: any_usage.can_infer_type,
                code_context: Some(any_usage.context.clone()),
            });
        }
        
        // Check for unsafe type assertions
        for assertion in &file.type_assertions {
            if !assertion.is_safe {
                issues.push(Issue {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: Severity::High,
                    category: CheckCategory::TypeSafety,
                    file: file.path.clone(),
                    line: Some(assertion.line),
                    column: Some(assertion.column),
                    description: "Unsafe type assertion without validation".to_string(),
                    impact: "Runtime error if type assumption is wrong".to_string(),
                    suggestion: "Add type guard or runtime validation".to_string(),
                    auto_fixable: false,
                    code_context: Some(assertion.context.clone()),
                });
            }
        }
        
        // Check for missing return types
        for func in &file.functions {
            if func.return_type.is_none() && func.should_have_return_type {
                issues.push(Issue {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: Severity::Low,
                    category: CheckCategory::TypeSafety,
                    file: file.path.clone(),
                    line: Some(func.line),
                    column: None,
                    description: format!("Function '{}' missing return type", func.name),
                    impact: "Reduced type inference, less IDE support".to_string(),
                    suggestion: "Add explicit return type annotation".to_string(),
                    auto_fixable: func.can_infer_return_type,
                    code_context: Some(func.context.clone()),
                });
            }
        }
        
        Ok(issues)
    }
    
    fn check_security(&self, file: &ParsedFile) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        
        // Check for XSS vulnerabilities
        for jsx_usage in &file.jsx_dangerous_usages {
            issues.push(Issue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::Critical,
                category: CheckCategory::Security,
                file: file.path.clone(),
                line: Some(jsx_usage.line),
                column: Some(jsx_usage.column),
                description: "Potential XSS vulnerability".to_string(),
                impact: "User input rendered without sanitization, code injection risk".to_string(),
                suggestion: "Use textContent or sanitize with DOMPurify".to_string(),
                auto_fixable: false,
                code_context: Some(jsx_usage.context.clone()),
            });
        }
        
        // Check for hardcoded secrets
        for secret in &file.hardcoded_secrets {
            issues.push(Issue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::High,
                category: CheckCategory::Security,
                file: file.path.clone(),
                line: Some(secret.line),
                column: None,
                description: format!("{} hardcoded in source", secret.secret_type),
                impact: "Credentials exposed in code repository".to_string(),
                suggestion: "Move to environment variable".to_string(),
                auto_fixable: false,
                code_context: Some(secret.context.clone()),
            });
        }
        
        // Check for unsafe eval/Function usage
        for eval_usage in &file.eval_usages {
            issues.push(Issue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::Critical,
                category: CheckCategory::Security,
                file: file.path.clone(),
                line: Some(eval_usage.line),
                column: None,
                description: "Use of eval() or Function constructor".to_string(),
                impact: "Code injection vulnerability".to_string(),
                suggestion: "Refactor to avoid dynamic code execution".to_string(),
                auto_fixable: false,
                code_context: Some(eval_usage.context.clone()),
            });
        }
        
        Ok(issues)
    }
    
    fn check_performance(&self, file: &ParsedFile) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        
        // Check for missing memoization (React)
        for computation in &file.expensive_computations {
            if !computation.is_memoized && computation.should_be_memoized {
                issues.push(Issue {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: Severity::Medium,
                    category: CheckCategory::Performance,
                    file: file.path.clone(),
                    line: Some(computation.line),
                    column: None,
                    description: "Expensive computation without memoization".to_string(),
                    impact: "Re-computes on every render, poor performance".to_string(),
                    suggestion: "Wrap in useMemo or useCallback".to_string(),
                    auto_fixable: true,
                    code_context: Some(computation.context.clone()),
                });
            }
        }
        
        // Check for inefficient loops
        for loop_pattern in &file.inefficient_loops {
            issues.push(Issue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::Medium,
                category: CheckCategory::Performance,
                file: file.path.clone(),
                line: Some(loop_pattern.line),
                column: None,
                description: format!("Inefficient loop pattern (O({}))", loop_pattern.complexity),
                impact: "Poor performance with large datasets".to_string(),
                suggestion: loop_pattern.suggestion.clone(),
                auto_fixable: false,
                code_context: Some(loop_pattern.context.clone()),
            });
        }
        
        Ok(issues)
    }
    
    fn check_code_smells(&self, file: &ParsedFile) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        
        // Check function length
        for func in &file.functions {
            if func.lines > 100 {
                issues.push(Issue {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: Severity::Medium,
                    category: CheckCategory::CodeSmells,
                    file: file.path.clone(),
                    line: Some(func.line),
                    column: None,
                    description: format!("Function too long ({} lines)", func.lines),
                    impact: "Reduced maintainability, harder to test".to_string(),
                    suggestion: "Extract into smaller functions".to_string(),
                    auto_fixable: false,
                    code_context: Some(func.context.clone()),
                });
            } else if func.lines > 50 {
                issues.push(Issue {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: Severity::Low,
                    category: CheckCategory::CodeSmells,
                    file: file.path.clone(),
                    line: Some(func.line),
                    column: None,
                    description: format!("Function is long ({} lines)", func.lines),
                    impact: "May be harder to maintain".to_string(),
                    suggestion: "Consider breaking into smaller functions".to_string(),
                    auto_fixable: false,
                    code_context: Some(func.context.clone()),
                });
            }
        }
        
        // Check for magic numbers
        for magic_number in &file.magic_numbers {
            issues.push(Issue {
                id: uuid::Uuid::new_v4().to_string(),
                severity: Severity::Low,
                category: CheckCategory::CodeSmells,
                file: file.path.clone(),
                line: Some(magic_number.line),
                column: Some(magic_number.column),
                description: format!("Magic number '{}'", magic_number.value),
                impact: "Unclear meaning, reduced maintainability".to_string(),
                suggestion: format!("Extract to named constant (e.g., {})", magic_number.suggested_name),
                auto_fixable: true,
                code_context: Some(magic_number.context.clone()),
            });
        }
        
        // Check cyclomatic complexity
        for func in &file.functions {
            if func.complexity > 15 {
                issues.push(Issue {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: Severity::Medium,
                    category: CheckCategory::CodeSmells,
                    file: file.path.clone(),
                    line: Some(func.line),
                    column: None,
                    description: format!("High complexity ({})", func.complexity),
                    impact: "Hard to test, error-prone".to_string(),
                    suggestion: "Simplify logic or extract functions".to_string(),
                    auto_fixable: false,
                    code_context: Some(func.context.clone()),
                });
            }
        }
        
        Ok(issues)
    }
    
    fn check_error_handling(&self, file: &ParsedFile) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();
        
        // Check async operations without error handling
        for async_call in &file.async_calls {
            if !async_call.has_error_handling {
                issues.push(Issue {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: Severity::High,
                    category: CheckCategory::ErrorHandling,
                    file: file.path.clone(),
                    line: Some(async_call.line),
                    column: None,
                    description: "Async operation without error handling".to_string(),
                    impact: "Silent failures, poor user experience".to_string(),
                    suggestion: "Add try-catch or .catch()".to_string(),
                    auto_fixable: true,
                    code_context: Some(async_call.context.clone()),
                });
            }
        }
        
        // Check for unsafe property access
        for access in &file.property_accesses {
            if !access.is_safe {
                issues.push(Issue {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: Severity::Medium,
                    category: CheckCategory::ErrorHandling,
                    file: file.path.clone(),
                    line: Some(access.line),
                    column: Some(access.column),
                    description: "Potential null/undefined access".to_string(),
                    impact: "Runtime error if value is null/undefined".to_string(),
                    suggestion: "Use optional chaining (?.) or null check".to_string(),
                    auto_fixable: true,
                    code_context: Some(access.context.clone()),
                });
            }
        }
        
        Ok(issues)
    }
    
    async fn auto_fix_issues(
        &self,
        files: &mut [ParsedFile],
        issues: &[Issue],
    ) -> Result<Vec<AppliedFix>> {
        let mut applied_fixes = Vec::new();
        let fixable_issues: Vec<_> = issues.iter()
            .filter(|i| i.auto_fixable)
            .collect();
        
        self.emit_reasoning(&format!(
            "Attempting to fix {} issues",
            fixable_issues.len()
        ));
        
        for issue in fixable_issues {
            self.emit_reasoning(&format!(
                "Fixing: {} in {}",
                issue.description, issue.file
            ));
            
            match self.fixer.apply_fix(files, issue).await {
                Ok(fix) => {
                    self.emit_reasoning(&format!("✓ Fixed: {}", issue.description));
                    applied_fixes.push(fix);
                }
                Err(e) => {
                    self.emit_reasoning(&format!(
                        "✗ Failed to fix: {} - {}",
                        issue.description, e
                    ));
                }
            }
        }
        
        Ok(applied_fixes)
    }
    
    fn calculate_confidence(
        &self,
        issues: &[Issue],
        files: &[ParsedFile],
    ) -> ConfidenceBreakdown {
        // Count issues by severity
        let critical_count = issues.iter().filter(|i| matches!(i.severity, Severity::Critical)).count();
        let high_count = issues.iter().filter(|i| matches!(i.severity, Severity::High)).count();
        let medium_count = issues.iter().filter(|i| matches!(i.severity, Severity::Medium)).count();
        let low_count = issues.iter().filter(|i| matches!(i.severity, Severity::Low)).count();
        
        // Calculate penalties
        let issue_penalty = (
            critical_count as f32 * 0.30 +
            high_count as f32 * 0.15 +
            medium_count as f32 * 0.05 +
            low_count as f32 * 0.01
        ).min(0.80);
        
        // Calculate individual category scores
        let code_quality = self.calculate_code_quality_score(files, issues);
        let type_safety = self.calculate_type_safety_score(files, issues);
        let security = self.calculate_security_score(files, issues);
        let performance = self.calculate_performance_score(files, issues);
        let maintainability = self.calculate_maintainability_score(files, issues);
        
        // Overall weighted score
        let overall = (
            code_quality * 0.25 +
            type_safety * 0.25 +
            security * 0.25 +
            performance * 0.15 +
            maintainability * 0.10
        ) * (1.0 - issue_penalty);
        
        ConfidenceBreakdown {
            overall: (overall * 100.0).clamp(0.0, 100.0),
            code_quality: (code_quality * 100.0),
            type_safety: (type_safety * 100.0),
            security: (security * 100.0),
            performance: (performance * 100.0),
            maintainability: (maintainability * 100.0),
        }
    }
    
    fn emit_reasoning(&self, content: &str) {
        // Emit via Tauri events
    }
    
    fn emit_progress(&self, progress: f32) {
        // Update progress bar
    }
    
    fn emit_complete(&self, report: &QualityReport) {
        // Notify completion
    }
    
    fn emit_error(&self, report: &QualityReport) {
        // Notify error state
    }
}

Testing Quality Agent
Unit Tests
rust#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_type_safety_check() {
        let agent = create_test_quality_agent();
        let file = create_file_with_any_types();
        
        let issues = agent.check_type_safety(&file).unwrap();
        
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|i| 
            i.description.contains("any")
        ));
    }
    
    #[tokio::test]
    async fn test_security_check_xss() {
        let agent = create_test_quality_agent();
        let file = create_file_with_dangerous_html();
        
        let issues = agent.check_security(&file).unwrap();
        
        assert!(!issues.is_empty());
        assert_eq!(issues[0].severity, Severity::Critical);
        assert!(issues[0].description.contains("XSS"));
    }
    
    #[tokio::test]
    async fn test_auto_fix_unused_imports() {
        let mut agent = create_test_quality_agent();
        let mut files = vec![create_file_with_unused_imports()];
        let issues = vec![create_unused_import_issue()];
        
        let fixes = agent.auto_fix_issues(&mut files, &issues).await.unwrap();
        
        assert_eq!(fixes.len(), 1);
        assert!(!files[0].content.contains("unused import"));
    }
    
    #[tokio::test]
    async fn test_confidence_calculation() {
        let agent = create_test_quality_agent();
        
        // No issues = high confidence
        let issues = vec![];
        let files = create_clean_files();
        let confidence = agent.calculate_confidence(&issues, &files);
        assert!(confidence.overall > 95.0);
        
        // Critical issues = low confidence
        let issues = vec![create_critical_issue()];
        let confidence = agent.calculate_confidence(&issues, &files);
        assert!(confidence.overall < 50.0);
    }
    
    #[tokio::test]
    async fn test_full_review() {
        let mut agent = create_test_quality_agent();
        let engineer_output = create_test_code_output();
        
        let report = agent.review_code(engineer_output).await.unwrap();
        
        assert!(report.files_reviewed > 0);
        assert!(report.confidence.overall > 0.0);
    }
    
    #[tokio::test]
    async fn test_blocking_on_critical() {
        let mut agent = create_test_quality_agent();
        let output_with_critical = create_code_with_critical_issues();
        
        let result = agent.review_code(output_with_critical).await;
        
        assert!(result.is_err());
        assert!(matches!(agent.state, QualityState::Error { .. }));
    }
}

Success Metrics
Performance Targets

Review speed: < 15 seconds for typical project
Analysis phase: < 4 seconds
Per-file review: < 1 second average
Auto-fix: < 5 seconds total

Quality Targets

Bug detection rate: > 95% (critical bugs)
False positive rate: < 5%
Auto-fix success rate: > 90%
Security issue detection: 100% (known patterns)

Confidence Targets

Average confidence: > 85%
High confidence projects (>90%): > 70%
Low confidence projects (<70%): < 15%
Confidence accuracy: ±5% of actual quality

User Experience Targets

Issue clarity: > 4.5/5 user rating
Fix suggestions usefulness: > 4.0/5
Review time satisfaction: > 4.0/5
False positive frustration: < 2.0/5


Integration with Pipeline
Input from Engineer
ruststruct CodeOutput {
    files: Vec<GeneratedFile>,
    total_lines: usize,
    confidence: ConfidenceBreakdown,
    quality_report: QualityReport,
}
Output to Debug
ruststruct QualityReport {
    files_reviewed: usize,
    issues: Vec<Issue>,
    fixes_applied: Vec<AppliedFix>,
    confidence: ConfidenceBreakdown,
    validated_files: Vec<ParsedFile>, // For Debug to test
}
Communication Events
Quality → Frontend:

quality:analyzing - Started analysis
quality:reviewing - Reviewing file X of Y
quality:issue_found - Issue detected
quality:fixing - Applying fix
quality:progress - Progress update (0-100%)
quality:complete - Review complete
quality:error - Critical issues found

Frontend → Quality:

quality:start - Begin review
quality:pause - Pause review
quality:skip_file - Skip current file
quality:view_issue - Request issue details
quality:override - Override blocking issue (dangerous)


Configuration Options
User-Configurable Settings
rustpub struct QualityConfig {
    // Strictness
    pub strictness: StrictnessLevel, // Relaxed, Standard, Strict
    
    // Auto-fix
    pub auto_fix_enabled: bool,
    pub max_auto_fixes: usize,
    pub auto_fix_categories: Vec<CheckCategory>,
    
    // Thresholds
    pub confidence_threshold: f32,
    pub max_function_lines: usize,
    pub max_complexity: usize,
    pub max_line_length: usize,
    
    // Check toggles
    pub check_categories: Vec<CheckCategory>,
    pub ignore_patterns: Vec<String>, // File patterns to ignore
    
    // Standards
    pub coding_standards: CodingStandards,
}

pub struct CodingStandards {
    pub indent_style: IndentStyle, // Spaces, Tabs
    pub indent_size: usize,
    pub quotes: QuoteStyle, // Single, Double
    pub semicolons: bool,
    pub trailing_commas: bool,
    pub line_ending: LineEnding, // LF, CRLF
}

Edge Cases & Error Handling
Unparseable Files
rustif parse_result.is_err() {
    issues.push(Issue {
        severity: Severity::Critical,
        category: CheckCategory::Syntax,
        description: "File cannot be parsed".to_string(),
        impact: "Build will fail".to_string(),
        suggestion: "Check syntax errors".to_string(),
        auto_fixable: false,
    });
    return Err(anyhow::anyhow!("Parse error"));
}
Timeout Protection
rustlet review_timeout = Duration::from_secs(30);
let result = tokio::time::timeout(
    review_timeout,
    self.check_file(file)
).await??;
LLM Failure Fallback
rust// If LLM-based check fails, use static analysis fallback
let issues = match self.llm_deep_check(file).await {
    Ok(issues) => issues,
    Err(e) => {
        self.emit_reasoning("LLM check failed, using static analysis");
        self.static_analysis_fallback(file)?
    }
};
Memory Protection
rustconst MAX_FILE_SIZE: usize = 1_000_000; // 1MB

if file.content.len() > MAX_FILE_SIZE {
    return Err(anyhow::anyhow!("File too large for quality check"));
}

Future Enhancements
Learning from Fixes

Track which auto-fixes users accept/reject
Improve fix algorithms based on feedback
Personalize checks to team preferences

Custom Rules

Allow users to define custom quality rules
Plugin system for domain-specific checks
Team-shared rule sets

AI-Powered Suggestions

Context-aware refactoring suggestions
Architecture improvement recommendations
Performance optimization ideas

Integration Features

Export reports to CI/CD
Integration with external linters (ESLint, etc.)
Code coverage integration