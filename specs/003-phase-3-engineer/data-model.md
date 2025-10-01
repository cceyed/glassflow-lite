# Data Model: Engineer Agent System

**Feature**: 003-phase-3-engineer  
**Date**: 2025-10-01  
**Status**: Complete

## Overview
Data model for the Engineer Agent, which generates code from architecture plans. Defines 9 key entities with relationships, validation rules, and state transitions.

---

## Entity Definitions

### 1. EngineerState
**Purpose**: Represents the current state of the Engineer Agent

**States**:
```rust
enum EngineerState {
    Idle,
    AnalyzingPlan { plan: ArchitecturePlan },
    GeneratingCode {
        plan: ArchitecturePlan,
        files_completed: Vec<GeneratedFile>,
        current_file: Option<String>,
        progress: f32,
    },
    Reviewing { generated_files: Vec<GeneratedFile> },
    Complete {
        output: CodeOutput,
        duration: Duration,
    },
    Error {
        message: String,
        failed_file: Option<String>,
        recoverable: bool,
    },
}
```

**Valid Transitions**:
- `Idle → AnalyzingPlan` (on plan received)
- `AnalyzingPlan → GeneratingCode` (plan valid)
- `AnalyzingPlan → Error` (plan invalid)
- `GeneratingCode → Reviewing` (all files generated)
- `GeneratingCode → Error` (generation failed)
- `Reviewing → Complete` (review passed)
- `Reviewing → GeneratingCode` (issues found, retry)
- `Complete → Idle` (auto-transition after handoff)
- `Error → Idle` (user cancels)
- `Error → AnalyzingPlan` (retry with modified plan)

**Validation Rules**:
- Cannot transition from `Idle` directly to `Complete`
- `Error` state must specify if recoverable
- `GeneratingCode` progress must be 0.0-1.0

---

### 2. ArchitecturePlan
**Purpose**: Input from Architect Agent containing complete architecture specification

**Fields**:
```rust
struct ArchitecturePlan {
    project_name: String,
    project_intent: ProjectIntent,
    tech_stack: TechStack,
    architecture_pattern: ArchitecturePattern,
    file_structure: FileStructure,
    component_hierarchy: Vec<Component>,
    dependencies: Vec<Dependency>,
    architecture_decisions: Vec<ArchitectureDecision>,
    confidence_breakdown: ConfidenceBreakdown,
}

struct FileStructure {
    directories: Vec<Directory>,
    files: Vec<FileTemplate>,
}

struct TechStack {
    language: String,           // e.g., "TypeScript"
    runtime: String,            // e.g., "Node.js 18+"
    framework: Option<String>,  // e.g., "React 18"
    styling: Option<String>,    // e.g., "TailwindCSS"
    state_management: Option<String>,
    testing: Option<String>,
}

struct Dependency {
    name: String,               // e.g., "react"
    version: String,            // e.g., "^18.0.0"
    dev_only: bool,
}
```

**Validation Rules**:
- `project_name` must be non-empty
- `file_structure.files` must have at least 1 file
- All `FileTemplate` paths must be unique
- All `dependencies` must have valid semver versions
- `tech_stack.language` must be specified

**Relationships**:
- Contains multiple `FileTemplate` (1:N)
- Contains multiple `Dependency` (1:N)
- Produced by Architect Agent (upstream)

---

### 3. FileTemplate
**Purpose**: Specification for a single file to be generated

**Fields**:
```rust
struct FileTemplate {
    path: String,                    // e.g., "src/components/TodoList.tsx"
    purpose: String,                 // e.g., "Main todo list component"
    estimated_lines: usize,
    language: Language,              // TypeScript, JavaScript, CSS, etc.
    dependencies: Vec<String>,       // Other files this depends on
    generation_hints: Vec<String>,   // Optional hints for code generation
}

enum Language {
    TypeScript,
    JavaScript,
    CSS,
    JSON,
    Markdown,
}
```

**Validation Rules**:
- `path` must be valid relative path
- `estimated_lines` must be > 0
- `dependencies` must reference valid file paths or external packages
- No circular dependencies allowed

**Relationships**:
- Part of `ArchitecturePlan` (N:1)
- Generates one `GeneratedFile` (1:1)

---

### 4. GeneratedFile
**Purpose**: Output file containing generated code

**Fields**:
```rust
struct GeneratedFile {
    path: String,
    content: String,
    language: Language,
    lines: usize,
    imports: Vec<Import>,
    exports: Vec<Export>,
    types: Vec<TypeDefinition>,
    confidence: f32,              // 0.0-1.0
}

struct Import {
    source: String,               // e.g., "react" or "./TodoItem"
    items: Vec<String>,           // e.g., ["useState", "useEffect"]
    is_type_only: bool,
}

struct Export {
    name: String,
    export_type: ExportType,      // Default, Named, Type
}

struct TypeDefinition {
    name: String,
    kind: TypeKind,               // Interface, Type, Enum, Class
}
```

**Validation Rules**:
- `content` must be non-empty
- `lines` must match actual line count
- `confidence` must be 0.0-1.0
- All `imports` must be resolvable
- All `exports` must be valid

**Relationships**:
- Generated from `FileTemplate` (1:1)
- Part of `CodeOutput` (N:1)
- Validated by `CodeQualityCheck` (1:1)

---

### 5. CodeQualityCheck
**Purpose**: Quality assessment results for generated code

**Fields**:
```rust
struct CodeQualityCheck {
    syntax_valid: bool,
    types_correct: bool,
    imports_resolved: bool,
    exports_valid: bool,
    style_compliant: bool,
    edge_cases_handled: bool,
    documented: bool,
    issues: Vec<QualityIssue>,
}
```

**Validation Rules**:
- All boolean fields default to `false` (must be explicitly validated)
- `issues` list must be populated if any check fails

**Relationships**:
- Validates one `GeneratedFile` (1:1)
- Contains multiple `QualityIssue` (1:N)

---

### 6. QualityIssue
**Purpose**: Individual code quality issue

**Fields**:
```rust
struct QualityIssue {
    severity: Severity,
    category: QualityCategory,
    line: Option<usize>,
    description: String,
    suggestion: String,
}

enum Severity {
    Critical,   // Blocks compilation
    High,       // Major problem
    Medium,     // Minor problem
    Low,        // Style/convention
}

enum QualityCategory {
    Syntax,
    Type,
    Import,
    Export,
    Style,
    Logic,
    Performance,
    Security,
}
```

**Validation Rules**:
- `description` must be non-empty
- `suggestion` must provide actionable fix
- `line` should be specified when applicable

**Relationships**:
- Part of `CodeQualityCheck` (N:1)

**Auto-Fix Eligibility**:
- `Low` severity: Always auto-fixable (formatting, style)
- `Medium` severity: Sometimes auto-fixable (simple type improvements)
- `High` severity: Rarely auto-fixable (requires logic changes)
- `Critical` severity: Never auto-fixable (requires user intervention)

---

### 7. ConfidenceBreakdown
**Purpose**: Detailed confidence metrics for generated code

**Fields**:
```rust
struct ConfidenceBreakdown {
    overall: f32,              // 0.0-100.0
    quality_score: f32,        // 0.0-100.0
    plan_adherence: f32,       // 0.0-100.0
    issue_penalty: f32,        // 0.0-100.0
}
```

**Calculation Formula**:
```
overall = (quality_score * 0.50) + 
          (plan_adherence * 0.30) + 
          ((100.0 - issue_penalty) * 0.20)

quality_score = (passed_checks / total_checks) * 100.0

plan_adherence = (files_matching_spec / total_files) * 100.0

issue_penalty = sum(severity_penalties), capped at 80.0
  - Critical: 20.0 per issue
  - High: 10.0 per issue
  - Medium: 5.0 per issue
  - Low: 2.0 per issue
```

**Validation Rules**:
- All scores must be 0.0-100.0
- `overall` must match calculated value

**Relationships**:
- Part of `CodeOutput` (1:1)

---

### 8. ReasoningEntry
**Purpose**: Real-time reasoning log entry for transparency

**Fields**:
```rust
struct ReasoningEntry {
    timestamp: DateTime<Utc>,
    phase: ReasoningPhase,
    entry_type: ReasoningType,
    content: String,
    confidence: Option<f32>,
}

enum ReasoningPhase {
    Analyzing,
    Generating,
    Reviewing,
}

enum ReasoningType {
    Observation,
    Analysis,
    Decision,
    ProgressUpdate,
    IssueFound,
    FixApplied,
}
```

**Validation Rules**:
- `content` must be non-empty
- `timestamp` must be in chronological order
- `confidence` only applicable for certain types

**Relationships**:
- Emitted during all state transitions
- Streamed to frontend in real-time

---

### 9. CodeOutput
**Purpose**: Final output bundle containing all generated code

**Fields**:
```rust
struct CodeOutput {
    files: Vec<GeneratedFile>,
    total_lines: usize,
    confidence: ConfidenceBreakdown,
    quality_report: QualityReport,
    generation_metadata: GenerationMetadata,
}

struct QualityReport {
    total_files: usize,
    files_passed: usize,
    files_failed: usize,
    issues_found: usize,
    issues_fixed: usize,
    critical_issues: Vec<QualityIssue>,
}

struct GenerationMetadata {
    started_at: DateTime<Utc>,
    completed_at: DateTime<Utc>,
    duration: Duration,
    files_generated_concurrently: usize,
    timeouts_encountered: usize,
}
```

**Validation Rules**:
- `files` must not be empty
- `total_lines` must match sum of file lines
- `quality_report.total_files` must match `files.len()`

**Relationships**:
- Contains multiple `GeneratedFile` (1:N)
- Contains one `ConfidenceBreakdown` (1:1)
- Passed to Quality Agent (downstream)

---

## Entity Relationship Diagram

```
ArchitecturePlan (from Architect Agent)
    ├─→ FileTemplate (1:N)
    │       └─→ GeneratedFile (1:1)
    │               ├─→ CodeQualityCheck (1:1)
    │               │       └─→ QualityIssue (1:N)
    │               └─→ CodeOutput (N:1)
    │                       ├─→ ConfidenceBreakdown (1:1)
    │                       └─→ QualityReport (1:1)
    └─→ Dependency (1:N)

EngineerState (state machine)
    ├─→ ArchitecturePlan (input)
    ├─→ GeneratedFile (intermediate)
    └─→ CodeOutput (output)

ReasoningEntry (emitted throughout)
```

---

## State Lifecycle

```
1. IDLE
   ↓ (receive ArchitecturePlan)
2. ANALYZING_PLAN
   ↓ (validate plan, build dependency graph)
3. GENERATING_CODE
   ↓ (generate all FileTemplate → GeneratedFile)
   │ (concurrent generation for independent files)
   │ (timeout handling: prompt user after 2min)
4. REVIEWING
   ↓ (run CodeQualityCheck on all GeneratedFile)
   │ (auto-fix Low/Medium severity QualityIssue)
   │ (re-validate after fixes)
5. COMPLETE
   ↓ (package into CodeOutput)
   │ (calculate ConfidenceBreakdown)
   │ (generate QualityReport)
6. → Quality Agent (downstream)

ERROR (can occur at any step)
   ↓ (preserve completed files only)
   → User decision: Retry / Modify Plan / Cancel
```

---

## Validation Summary

**Critical Validations**:
1. No circular dependencies in FileTemplate
2. All imports resolve to generated files or plan dependencies
3. All generated code has valid syntax
4. Confidence scores within 0.0-1.0 range
5. State transitions follow valid paths only

**Quality Gates**:
1. Syntax validation (TypeScript Compiler API)
2. Type checking (no 'any' types)
3. Import resolution (against plan dependencies)
4. Style compliance (ESLint)
5. Edge case handling (null safety, error handling)
6. Documentation completeness (JSDoc for public APIs)

**Performance Constraints**:
1. File generation timeout: 2 minutes (with user prompt)
2. Concurrent generation: Max 3-5 files simultaneously
3. Total generation: No hard limit (user can cancel)

---

## Data Persistence

**Persisted Data**:
- Generation progress (for resumption after interruption)
- Completed files (atomic writes to configurable directory)
- Quality reports (for user review)

**Not Persisted**:
- Partial/incomplete files (discarded on error)
- Intermediate state (regenerated on retry)
- Reasoning entries (streamed only, not stored)

---

**Status**: Data model complete. Ready for contract generation.
