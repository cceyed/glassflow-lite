# Research: Engineer Agent System

**Feature**: 003-phase-3-engineer  
**Date**: 2025-10-01  
**Status**: Complete

## Overview
Research findings for implementing the Engineer Agent, which generates clean, functional code from architecture plans. All technical decisions documented with rationale and alternatives.

---

## 1. Concurrent Code Generation Patterns

### Decision
Use **Tokio async runtime with semaphore-based concurrency control**

### Rationale
- Tokio is the de facto standard for async Rust, well-tested and maintained
- Semaphore pattern allows limiting concurrent LLM API calls (e.g., max 3-5 concurrent)
- Non-blocking I/O prevents thread exhaustion
- Integrates seamlessly with existing Tauri/async ecosystem
- Provides timeout support via `tokio::time::timeout`

### Implementation Approach
```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

let semaphore = Arc::new(Semaphore::new(3)); // Max 3 concurrent
for file in independent_files {
    let permit = semaphore.clone().acquire_owned().await?;
    tokio::spawn(async move {
        let _permit = permit; // Hold until task completes
        generate_file(file).await
    });
}
```

### Alternatives Considered
- **Thread pool**: More complex lifecycle management, blocking I/O
- **Sequential only**: Simpler but 3-5x slower for large projects
- **Rayon parallel iterator**: Better for CPU-bound tasks, not async I/O

### References
- Tokio semaphore docs: https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html
- Async patterns in Rust: https://rust-lang.github.io/async-book/

---

## 2. Code Quality Validation Approaches

### Decision
Use **TypeScript Compiler API for syntax/type checking + ESLint for style**

### Rationale
- TypeScript Compiler API is the official parser, 100% accurate
- Provides full AST access for deep validation (imports, exports, types)
- ESLint is industry standard for style/best practices
- Both tools have Node.js bindings callable from Rust via `Command`
- Avoids reinventing complex parsing logic

### Implementation Approach
```rust
// Syntax validation via tsc
let output = Command::new("npx")
    .args(&["tsc", "--noEmit", "--strict", file_path])
    .output()?;

// Style validation via ESLint
let output = Command::new("npx")
    .args(&["eslint", "--format=json", file_path])
    .output()?;
```

### Alternatives Considered
- **Custom TypeScript parser**: Massive effort, error-prone, hard to maintain
- **Regex-based validation**: Fragile, misses complex cases (nested types, generics)
- **swc parser**: Fast but less mature than official TypeScript tooling

### Quality Checks Implemented
1. **Syntax**: Valid TypeScript/JavaScript syntax
2. **Types**: No `any` types, explicit return types, type correctness
3. **Imports**: All imports resolve (checked against architecture plan dependencies)
4. **Exports**: Valid export statements
5. **Style**: ESLint rules (formatting, naming conventions)
6. **Edge cases**: Null safety (optional chaining), async error handling
7. **Documentation**: JSDoc comments for public APIs

### References
- TypeScript Compiler API: https://github.com/microsoft/TypeScript/wiki/Using-the-Compiler-API
- ESLint Node.js API: https://eslint.org/docs/latest/integrate/nodejs-api

---

## 3. Timeout Handling Strategy

### Decision
Use **Tokio timeout with user prompt on expiration (2 minutes per file)**

### Rationale
- Non-blocking: Doesn't freeze UI or block other operations
- User control: Allows user to decide whether to continue or cancel
- Prevents infinite hangs from LLM API issues
- Configurable threshold (default 2min, adjustable)
- Graceful degradation: Can skip problematic file and continue

### Implementation Approach
```rust
use tokio::time::{timeout, Duration};

match timeout(Duration::from_secs(120), generate_file(template)).await {
    Ok(Ok(file)) => Ok(file),
    Ok(Err(e)) => Err(e),
    Err(_) => {
        // Timeout expired - prompt user
        let response = prompt_user_timeout(template.path).await?;
        match response {
            UserChoice::Continue => generate_file(template).await, // Retry
            UserChoice::Cancel => Err(Error::Timeout),
            UserChoice::Skip => Ok(empty_file_placeholder(template)),
        }
    }
}
```

### Alternatives Considered
- **Hard abort after timeout**: Poor UX, loses partial progress
- **No timeout**: Risk of infinite hangs, poor user experience
- **Exponential backoff retry**: Doesn't address user control need

### Timeout Thresholds
- **Per-file generation**: 2 minutes (configurable)
- **Total generation**: No hard limit (user can cancel anytime)
- **LLM API call**: 30 seconds (with retry logic)

---

## 4. File Output Management

### Decision
Use **atomic writes with temp files + rename, user-configurable output directory**

### Rationale
- Atomic rename prevents partial file corruption (OS-level guarantee)
- Temp file approach ensures all-or-nothing writes
- Configurable directory gives user control (staging vs. direct)
- Supports "review before commit" workflow
- Preserves only completed files on error (per clarification Q4)

### Implementation Approach
```rust
use std::fs;
use tempfile::NamedTempFile;

// Write to temp file
let mut temp_file = NamedTempFile::new_in(&output_dir)?;
temp_file.write_all(generated_code.as_bytes())?;

// Atomic rename (all-or-nothing)
let final_path = output_dir.join(&file_template.path);
temp_file.persist(&final_path)?;
```

### Configuration
```rust
struct OutputConfig {
    directory: PathBuf,        // User-configurable (default: project root)
    create_dirs: bool,          // Auto-create missing directories
    overwrite_existing: bool,   // Prompt or overwrite
}
```

### Alternatives Considered
- **Direct writes**: Risk of partial files on crash/error
- **Fixed output directory**: Inflexible, doesn't support staging workflow
- **In-memory only**: Requires manual export, loses progress on crash

### Error Handling
- **Disk full**: Detect early, fail gracefully with clear message
- **Permission denied**: Check write permissions before generation
- **Partial generation**: Keep only completed files, delete temp files

---

## 5. Import Resolution Strategy

### Decision
**Validate against architecture plan's dependency list only (no runtime checks)**

### Rationale
- Architecture plan is the source of truth (validated by Architect Agent)
- Avoids filesystem coupling (works without package.json present)
- Faster validation (no I/O, just list lookup)
- Consistent with clarification Q5 (validate against plan dependencies)
- Prevents false positives from missing node_modules

### Implementation Approach
```rust
struct ImportValidator {
    plan_dependencies: HashSet<String>,  // From architecture plan
    generated_files: HashSet<PathBuf>,   // Files generated so far
}

impl ImportValidator {
    fn validate_import(&self, import_path: &str) -> ValidationResult {
        if import_path.starts_with("./") || import_path.starts_with("../") {
            // Relative import - check generated files
            self.validate_relative_import(import_path)
        } else {
            // External package - check plan dependencies
            let package_name = extract_package_name(import_path);
            if self.plan_dependencies.contains(package_name) {
                ValidationResult::Valid
            } else {
                ValidationResult::Warning(format!(
                    "Package '{}' not in architecture plan dependencies",
                    package_name
                ))
            }
        }
    }
}
```

### Validation Rules
1. **Relative imports** (`./`, `../`): Must resolve to generated file or architecture plan file
2. **External packages**: Must be in architecture plan's dependency list
3. **Node built-ins** (`fs`, `path`): Always allowed (no validation)
4. **Type-only imports**: Validated same as runtime imports

### Alternatives Considered
- **Check package.json at runtime**: Couples to filesystem, fails if package.json missing
- **Filesystem scan**: Slow, requires node_modules present
- **No validation**: Misses import errors, poor code quality

### Error Reporting
- **Missing relative import**: High severity (likely breaks compilation)
- **Missing external package**: Warning (may be intentional, user verifies)
- **Circular dependency**: Critical (detected during dependency graph analysis)

---

## 6. State Machine Reuse from Architect Agent

### Decision
**Reuse state machine pattern and infrastructure from Architect Agent**

### Rationale
- Proven pattern already implemented and tested
- Consistent UX across agents (same LED indicators, transitions)
- Reduces implementation effort and bugs
- Shared reasoning stream infrastructure

### Shared Components
- `AgentState` enum pattern (IDLE, ANALYZING, WORKING, REVIEWING, COMPLETE, ERROR)
- State transition validation logic
- Reasoning entry streaming
- Confidence calculation framework
- LED indicator mapping

### Engineer-Specific Adaptations
- State names: ANALYZING_PLAN, GENERATING_CODE (vs. Architect's ANALYZING, DESIGNING)
- Progress tracking: File-by-file progress (vs. Architect's phase progress)
- Confidence factors: Code quality, plan adherence, issue penalty (vs. Architect's spec clarity, feasibility)

---

## 7. LLM Integration for Code Generation

### Decision
**Reuse LLM client from Architect Agent with streaming support**

### Rationale
- Already implemented with retry logic and error handling
- Streaming provides real-time feedback (line-by-line code generation)
- Consistent API across agents
- Handles rate limiting and timeouts

### Code Generation Prompt Strategy
```
Generate {language} code for: {file_path}

Purpose: {file_purpose}

Architecture Context:
{architecture_summary}

Dependencies:
{dependency_list}

Style Guide:
- No 'any' types
- Explicit return types
- JSDoc comments for public APIs
- Optional chaining for null safety
- Try-catch for async operations

Generate ONLY the code, no explanations.
```

### Streaming Approach
- Stream code line-by-line to UI for real-time preview
- Buffer complete file before validation
- Show progress indicator (lines generated)

---

## Summary

All technical decisions finalized:
1. ✅ Concurrent generation: Tokio + semaphore
2. ✅ Quality validation: TypeScript Compiler API + ESLint
3. ✅ Timeout handling: 2min prompt with user choice
4. ✅ File output: Atomic writes, configurable directory
5. ✅ Import resolution: Architecture plan dependencies only
6. ✅ State machine: Reuse from Architect Agent
7. ✅ LLM integration: Reuse with streaming

**No unknowns remain. Ready for Phase 1 (Design & Contracts).**
