# Quality Agent Documentation

## Overview
The Quality Agent is the critical code reviewer in the Glassflow pipeline. It receives code from the Engineer Agent and performs comprehensive quality checks, identifying bugs, security issues, code smells, and standards violations.

## Implementation Status
✅ **Complete** - Core functionality implemented and validated

## Module Structure

```
src-tauri/src/agents/quality/
├── mod.rs           # Main Quality implementation
├── checker.rs       # Quality checks (10 categories)
├── fixer.rs         # Automatic issue fixing
└── confidence.rs    # Confidence score calculation
```

## Key Components

### 1. Quality (mod.rs)
Main agent orchestrator:
- **`review_code()`** - Main entry point for code review
- Coordinates checker, fixer, and confidence calculator
- Handles critical issue detection
- Generates comprehensive quality reports

### 2. QualityChecker (checker.rs)
Performs 10 categories of quality checks:
1. **Syntax & Parsing** - Valid code syntax
2. **Type Safety** - TypeScript type correctness
3. **Imports** - Import resolution and unused imports
4. **Code Smells** - Maintainability issues
5. **Security** - XSS, eval, hardcoded secrets
6. **Performance** - Missing memoization, inefficient loops
7. **Best Practices** - Naming conventions, patterns
8. **Standards** - Code style compliance
9. **Documentation** - JSDoc and comments
10. **Error Handling** - Try-catch, promise handling

### 3. AutoFixer (fixer.rs)
Automatically fixes minor issues:
- Removes unused imports
- Adds type annotations
- Wraps computations in useMemo
- Adds error handling
- Fixes naming conventions
- Extracts magic numbers

### 4. ConfidenceCalculator (confidence.rs)
Calculates multi-factor confidence scores:
- **Code Quality** (25%) - Code smell metrics
- **Type Safety** (25%) - Type correctness
- **Security** (25%) - Security vulnerability count
- **Performance** (15%) - Performance issue count
- **Maintainability** (10%) - File size and complexity

## Issue Severity System

### CRITICAL (Blocking)
- Prevents code from running
- Security vulnerabilities (XSS, eval)
- Syntax errors
- **Action**: Stops pipeline, requires fix

### HIGH (Major Issue)
- Significant bugs likely
- Unsafe type assertions
- Missing error handling
- **Action**: Flags for review, some auto-fixable

### MEDIUM (Should Fix)
- Code smells
- Missing memoization
- Implicit any types
- **Action**: Often auto-fixable

### LOW (Nice to Fix)
- Cosmetic issues
- Unused imports
- Minor style violations
- **Action**: Always auto-fixable or ignorable

## Configuration

```rust
pub struct QualityConfig {
    pub strictness: StrictnessLevel,  // Relaxed, Standard, Strict
    pub auto_fix_enabled: bool,       // Default: true
    pub max_auto_fixes: usize,        // Default: 50
    pub confidence_threshold: f32,    // Default: 70.0
}
```

## Usage Example

```rust
use crate::agents::quality::{Quality, QualityConfig, StrictnessLevel};
use crate::llm::client::create_llm_client();

// Create quality agent
let llm_client = create_llm_client();
let mut quality = Quality::new(llm_client);

// Or with custom config
let config = QualityConfig {
    strictness: StrictnessLevel::Strict,
    auto_fix_enabled: true,
    max_auto_fixes: 100,
    confidence_threshold: 85.0,
};
let mut quality = Quality::with_config(llm_client, config);

// Review code from Engineer
match quality.review_code(engineer_output).await {
    Ok(report) => {
        println!("Review complete!");
        println!("Confidence: {:.1}%", report.confidence.overall);
        println!("Issues found: {}", report.issues.len());
        println!("Issues fixed: {}", report.fixes_applied.len());
    }
    Err(e) => {
        eprintln!("Critical issues found: {}", e);
    }
}
```

## Quality Report Structure

```rust
pub struct QualityReport {
    pub files_reviewed: usize,
    pub total_lines: usize,
    pub issues: Vec<QualityIssue>,
    pub fixes_applied: Vec<AppliedFix>,
    pub confidence: QualityConfidenceBreakdown,
    pub duration: Duration,
}

pub struct QualityConfidenceBreakdown {
    pub overall: f32,
    pub code_quality: f32,
    pub type_safety: f32,
    pub security: f32,
    pub performance: f32,
    pub maintainability: f32,
}
```

## Validation

Run the validation script:
```bash
bash scripts/validate-quality.sh
```

This validates:
- ✅ Rust compilation
- ✅ Rust tests
- ✅ TypeScript compilation
- ✅ All required files present

## Blueprint Compliance

The implementation follows the Phase 4 blueprint (`phase4.md`):
- ✅ State machine (Idle → Analyzing → Reviewing → Fixing → Complete)
- ✅ 10 quality check categories
- ✅ Issue severity system (Critical, High, Medium, Low)
- ✅ Auto-fix system with safety checks
- ✅ Multi-factor confidence calculation
- ✅ Comprehensive quality reporting
- ✅ Critical issue blocking

## Key Features

### Comprehensive Checks
- 10 categories of quality checks
- Pattern-based detection
- Context-aware analysis
- Severity-based prioritization

### Smart Auto-Fixing
- Only fixes low/medium severity issues
- Never changes logic or behavior
- Re-validates after fixes
- Tracks confidence per fix

### Confidence Scoring
- Multi-factor weighted calculation
- Issue penalty system
- Category-specific scores
- Transparent breakdown

### Error Handling
- Critical issues block pipeline
- Partial results on failure
- Clear error messages
- Recoverable errors

## Next Steps

1. **IPC Integration**: Add quality commands to IPC layer
2. **Frontend UI**: Create quality report display components
3. **Enhanced Checks**: Add more sophisticated pattern detection
4. **LLM Integration**: Use LLM for complex issue detection
5. **Custom Rules**: Allow user-defined quality rules

## Comparison with Other Agents

| Feature | Architect | Engineer | Quality |
|---------|-----------|----------|---------|
| Input | User spec | Architecture plan | Generated code |
| Output | Architecture plan | Code files | Quality report |
| LLM Usage | Heavy | Heavy | Light |
| Auto-fix | No | Yes (during gen) | Yes (post-gen) |
| Blocking | On ambiguity | On critical errors | On critical issues |
| Confidence | Spec clarity | Code completeness | Code quality |

All three agents are now fully implemented and validated!
