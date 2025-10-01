# IPC Commands Contract: Engineer Agent

**Feature**: 003-phase-3-engineer  
**Date**: 2025-10-01  
**Protocol**: Tauri IPC (Rust ↔ TypeScript)

## Overview
Defines the command interface between the Rust backend (Engineer Agent) and React frontend. All commands follow Tauri's async command pattern with Result return types.

---

## Commands

### 1. start_code_generation

**Purpose**: Initiates code generation from an architecture plan

**Signature**:
```rust
#[tauri::command]
async fn start_code_generation(
    plan: ArchitecturePlan,
    config: GenerationConfig,
) -> Result<(), String>
```

**Request**:
```typescript
interface ArchitecturePlan {
  projectName: string;
  techStack: TechStack;
  fileStructure: FileStructure;
  dependencies: Dependency[];
}

interface GenerationConfig {
  outputDirectory: string;        // User-configurable output path
  maxConcurrent: number;          // Max concurrent file generation (default: 3)
  timeoutSeconds: number;         // Per-file timeout (default: 120)
}
```

**Response**:
```typescript
// Success: void (state changes emitted via events)
// Error: string (error message)
```

**Behavior**:
- Validates architecture plan completeness
- Transitions state from IDLE → ANALYZING_PLAN
- Emits `engineer:state_changed` event
- Returns immediately (generation happens async)

**Error Cases**:
- Plan validation failed: "Incomplete architecture plan: missing {field}"
- Already generating: "Generation already in progress"
- Invalid output directory: "Output directory not writable: {path}"

---

### 2. get_engineer_state

**Purpose**: Retrieves current Engineer Agent state

**Signature**:
```rust
#[tauri::command]
fn get_engineer_state() -> EngineerState
```

**Response**:
```typescript
type EngineerState =
  | { type: 'idle' }
  | { type: 'analyzing_plan'; plan: ArchitecturePlan }
  | {
      type: 'generating_code';
      plan: ArchitecturePlan;
      filesCompleted: GeneratedFile[];
      currentFile: string | null;
      progress: number; // 0.0-1.0
    }
  | { type: 'reviewing'; generatedFiles: GeneratedFile[] }
  | { type: 'complete'; output: CodeOutput; duration: number }
  | {
      type: 'error';
      message: string;
      failedFile: string | null;
      recoverable: boolean;
    };
```

**Behavior**:
- Returns current state synchronously
- No side effects

---

### 3. get_generation_progress

**Purpose**: Retrieves detailed generation progress

**Signature**:
```rust
#[tauri::command]
fn get_generation_progress() -> GenerationProgress
```

**Response**:
```typescript
interface GenerationProgress {
  filesCompleted: number;
  filesTotal: number;
  percentage: number; // 0-100
  currentFile: string | null;
  elapsedSeconds: number;
  estimatedRemainingSeconds: number | null;
  filesInProgress: string[]; // For concurrent generation
}
```

**Behavior**:
- Returns progress snapshot
- Available only in GENERATING_CODE state
- Returns empty/zero values in other states

---

### 4. cancel_generation

**Purpose**: Cancels ongoing code generation

**Signature**:
```rust
#[tauri::command]
async fn cancel_generation() -> Result<(), String>
```

**Response**:
```typescript
// Success: void
// Error: string
```

**Behavior**:
- Stops all ongoing file generation
- Preserves successfully completed files only
- Discards partial/incomplete files
- Transitions to IDLE state
- Emits `engineer:state_changed` event

**Error Cases**:
- Not generating: "No generation in progress"

---

### 5. retry_generation

**Purpose**: Retries generation from ERROR state, optionally with modified plan

**Signature**:
```rust
#[tauri::command]
async fn retry_generation(
    modified_plan: Option<ArchitecturePlan>,
) -> Result<(), String>
```

**Request**:
```typescript
// If null, retries with original plan
// If provided, uses modified plan
modifiedPlan?: ArchitecturePlan;
```

**Response**:
```typescript
// Success: void
// Error: string
```

**Behavior**:
- Only available in ERROR state
- If `modified_plan` provided, validates and uses it
- Otherwise, retries with original plan
- Transitions to ANALYZING_PLAN
- Emits `engineer:state_changed` event

**Error Cases**:
- Not in error state: "Cannot retry: not in error state"
- Modified plan invalid: "Invalid plan: {reason}"

---

### 6. get_quality_report

**Purpose**: Retrieves quality check results

**Signature**:
```rust
#[tauri::command]
fn get_quality_report() -> Option<QualityReport>
```

**Response**:
```typescript
interface QualityReport {
  totalFiles: number;
  filesPassed: number;
  filesFailed: number;
  issuesFound: number;
  issuesFixed: number;
  criticalIssues: QualityIssue[];
  fileReports: FileQualityReport[];
}

interface FileQualityReport {
  path: string;
  passed: boolean;
  issues: QualityIssue[];
}

interface QualityIssue {
  severity: 'critical' | 'high' | 'medium' | 'low';
  category: 'syntax' | 'type' | 'import' | 'export' | 'style' | 'logic' | 'performance' | 'security';
  line: number | null;
  description: string;
  suggestion: string;
}
```

**Behavior**:
- Returns `null` if no generation completed yet
- Available after REVIEWING or COMPLETE state
- Includes auto-fix results

---

### 7. export_generated_code

**Purpose**: Exports generated code to specified directory

**Signature**:
```rust
#[tauri::command]
async fn export_generated_code(
    directory: String,
) -> Result<ExportResult, String>
```

**Request**:
```typescript
directory: string; // Absolute path to export directory
```

**Response**:
```typescript
interface ExportResult {
  filesExported: number;
  totalBytes: number;
  exportPath: string;
}

// Error: string
```

**Behavior**:
- Only available in COMPLETE state
- Copies all generated files to specified directory
- Creates directory structure as needed
- Uses atomic writes (temp file + rename)

**Error Cases**:
- Not complete: "No generated code to export"
- Invalid directory: "Export directory not writable: {path}"
- Disk full: "Insufficient disk space"

---

### 8. get_confidence_breakdown

**Purpose**: Retrieves detailed confidence metrics

**Signature**:
```rust
#[tauri::command]
fn get_confidence_breakdown() -> Option<ConfidenceBreakdown>
```

**Response**:
```typescript
interface ConfidenceBreakdown {
  overall: number; // 0-100
  qualityScore: number; // 0-100
  planAdherence: number; // 0-100
  issuePenalty: number; // 0-100
}
```

**Behavior**:
- Returns `null` if no generation completed yet
- Available after COMPLETE state
- Shows breakdown of confidence calculation

---

### 9. handle_timeout_prompt

**Purpose**: Handles user response to file generation timeout

**Signature**:
```rust
#[tauri::command]
async fn handle_timeout_prompt(
    file_path: String,
    choice: TimeoutChoice,
) -> Result<(), String>
```

**Request**:
```typescript
enum TimeoutChoice {
  Continue = 'continue', // Continue waiting
  Cancel = 'cancel', // Cancel entire generation
  Skip = 'skip', // Skip this file, continue with others
}
```

**Response**:
```typescript
// Success: void
// Error: string
```

**Behavior**:
- Called when file generation exceeds 2 minutes
- `Continue`: Extends timeout, retries file
- `Cancel`: Cancels entire generation
- `Skip`: Skips file, continues with remaining files

---

## Error Handling

**Error Format**:
All errors returned as `Result<T, String>` where String contains:
```
"{ErrorType}: {detailed_message}"
```

**Error Types**:
- `ValidationError`: Input validation failed
- `StateError`: Invalid state for operation
- `IOError`: File system operation failed
- `TimeoutError`: Operation timed out
- `LLMError`: LLM API call failed

**Example**:
```typescript
try {
  await invoke('start_code_generation', { plan, config });
} catch (error) {
  // error is string: "ValidationError: Incomplete architecture plan: missing file_structure"
}
```

---

## Testing Contract

**Contract Tests** (must fail initially):
```rust
// tests/contract/engineer_commands_test.rs

#[tokio::test]
async fn test_start_code_generation_contract() {
    let plan = create_valid_plan();
    let config = GenerationConfig::default();
    
    let result = start_code_generation(plan, config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_code_generation_invalid_plan() {
    let plan = create_invalid_plan(); // Missing required fields
    let config = GenerationConfig::default();
    
    let result = start_code_generation(plan, config).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Incomplete architecture plan"));
}

#[test]
fn test_get_engineer_state_contract() {
    let state = get_engineer_state();
    assert!(matches!(state, EngineerState::Idle));
}

#[tokio::test]
async fn test_cancel_generation_when_not_generating() {
    let result = cancel_generation().await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No generation in progress"));
}
```

---

**Status**: IPC commands contract complete. Ready for event contract definition.
