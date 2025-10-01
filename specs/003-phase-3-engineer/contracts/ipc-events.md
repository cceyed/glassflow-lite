# IPC Events Contract: Engineer Agent

**Feature**: 003-phase-3-engineer  
**Date**: 2025-10-01  
**Protocol**: Tauri Events (Rust → TypeScript)

## Overview
Defines the event interface for real-time updates from the Rust backend to React frontend. All events follow Tauri's event emission pattern.

---

## Events

### 1. engineer:state_changed

**Purpose**: Notifies frontend of state transitions

**Payload**:
```typescript
interface StateChangedEvent {
  previousState: EngineerStateType;
  newState: EngineerStateType;
  timestamp: string; // ISO 8601
}

type EngineerStateType =
  | 'idle'
  | 'analyzing_plan'
  | 'generating_code'
  | 'reviewing'
  | 'complete'
  | 'error';
```

**Emission Triggers**:
- Any state transition
- Emitted immediately after transition

**Frontend Handler**:
```typescript
import { listen } from '@tauri-apps/api/event';

listen<StateChangedEvent>('engineer:state_changed', (event) => {
  console.log(`State: ${event.payload.previousState} → ${event.payload.newState}`);
  updateUI(event.payload.newState);
});
```

---

### 2. engineer:file_started

**Purpose**: Notifies when file generation begins

**Payload**:
```typescript
interface FileStartedEvent {
  path: string;
  index: number; // 1-based
  total: number;
  estimatedLines: number;
  timestamp: string;
}
```

**Emission Triggers**:
- File generation starts (sequential or concurrent)
- Emitted for each file

**Example**:
```json
{
  "path": "src/components/TodoList.tsx",
  "index": 12,
  "total": 17,
  "estimatedLines": 89,
  "timestamp": "2025-10-01T14:30:45Z"
}
```

---

### 3. engineer:file_completed

**Purpose**: Notifies when file generation completes

**Payload**:
```typescript
interface FileCompletedEvent {
  path: string;
  lines: number;
  confidence: number; // 0-100
  hasIssues: boolean;
  timestamp: string;
}
```

**Emission Triggers**:
- File generation completes successfully
- Emitted after quality checks

**Example**:
```json
{
  "path": "src/components/TodoList.tsx",
  "lines": 92,
  "confidence": 97.5,
  "hasIssues": false,
  "timestamp": "2025-10-01T14:31:15Z"
}
```

---

### 4. engineer:reasoning

**Purpose**: Streams reasoning entries for transparency

**Payload**:
```typescript
interface ReasoningEvent {
  phase: 'analyzing' | 'generating' | 'reviewing';
  type: 'observation' | 'analysis' | 'decision' | 'progress' | 'issue' | 'fix';
  content: string;
  confidence?: number; // Optional, 0-100
  timestamp: string;
}
```

**Emission Triggers**:
- Key decisions made
- Issues detected
- Fixes applied
- Progress milestones

**Examples**:
```json
{
  "phase": "analyzing",
  "type": "observation",
  "content": "Architecture plan contains 17 files with dependency graph",
  "timestamp": "2025-10-01T14:30:00Z"
}

{
  "phase": "generating",
  "type": "progress",
  "content": "Generating TodoList.tsx (12/17 files, 71% complete)",
  "timestamp": "2025-10-01T14:30:45Z"
}

{
  "phase": "reviewing",
  "type": "issue",
  "content": "Found 2 minor style issues in TodoItem.tsx",
  "timestamp": "2025-10-01T14:32:00Z"
}

{
  "phase": "reviewing",
  "type": "fix",
  "content": "Auto-fixed 2 style issues in TodoItem.tsx",
  "confidence": 98.0,
  "timestamp": "2025-10-01T14:32:01Z"
}
```

---

### 5. engineer:progress

**Purpose**: Updates overall generation progress

**Payload**:
```typescript
interface ProgressEvent {
  completed: number;
  total: number;
  percentage: number; // 0-100
  elapsedSeconds: number;
  estimatedRemainingSeconds: number | null;
  currentFiles: string[]; // For concurrent generation
  timestamp: string;
}
```

**Emission Triggers**:
- After each file completes
- Every 5 seconds during generation (heartbeat)

**Example**:
```json
{
  "completed": 12,
  "total": 17,
  "percentage": 70.6,
  "elapsedSeconds": 45,
  "estimatedRemainingSeconds": 19,
  "currentFiles": ["src/components/TodoFilter.tsx", "src/components/TodoStats.tsx"],
  "timestamp": "2025-10-01T14:30:45Z"
}
```

---

### 6. engineer:error

**Purpose**: Notifies of errors during generation

**Payload**:
```typescript
interface ErrorEvent {
  message: string;
  failedFile: string | null;
  recoverable: boolean;
  errorType: 'validation' | 'generation' | 'timeout' | 'llm' | 'io';
  timestamp: string;
}
```

**Emission Triggers**:
- Generation error occurs
- Validation fails
- Timeout expires

**Examples**:
```json
{
  "message": "Failed to generate TodoList.tsx: LLM API rate limit exceeded",
  "failedFile": "src/components/TodoList.tsx",
  "recoverable": true,
  "errorType": "llm",
  "timestamp": "2025-10-01T14:31:00Z"
}

{
  "message": "Circular dependency detected: TodoList.tsx ↔ TodoItem.tsx",
  "failedFile": null,
  "recoverable": false,
  "errorType": "validation",
  "timestamp": "2025-10-01T14:30:10Z"
}
```

---

### 7. engineer:timeout_prompt

**Purpose**: Prompts user when file generation times out

**Payload**:
```typescript
interface TimeoutPromptEvent {
  filePath: string;
  elapsedSeconds: number;
  estimatedLines: number;
  timestamp: string;
}
```

**Emission Triggers**:
- File generation exceeds 2 minutes

**Frontend Handler**:
```typescript
listen<TimeoutPromptEvent>('engineer:timeout_prompt', async (event) => {
  const choice = await showTimeoutDialog(event.payload);
  await invoke('handle_timeout_prompt', {
    filePath: event.payload.filePath,
    choice, // 'continue' | 'cancel' | 'skip'
  });
});
```

---

### 8. engineer:quality_check_started

**Purpose**: Notifies when quality checks begin

**Payload**:
```typescript
interface QualityCheckStartedEvent {
  totalFiles: number;
  timestamp: string;
}
```

**Emission Triggers**:
- Transition to REVIEWING state

---

### 9. engineer:quality_check_completed

**Purpose**: Notifies when quality checks complete

**Payload**:
```typescript
interface QualityCheckCompletedEvent {
  totalFiles: number;
  filesPassed: number;
  filesFailed: number;
  issuesFound: number;
  issuesFixed: number;
  timestamp: string;
}
```

**Emission Triggers**:
- All quality checks complete
- After auto-fix attempts

---

### 10. engineer:generation_complete

**Purpose**: Notifies when entire generation completes

**Payload**:
```typescript
interface GenerationCompleteEvent {
  totalFiles: number;
  totalLines: number;
  confidence: number; // 0-100
  durationSeconds: number;
  timestamp: string;
}
```

**Emission Triggers**:
- Transition to COMPLETE state

**Example**:
```json
{
  "totalFiles": 17,
  "totalLines": 1247,
  "confidence": 95.3,
  "durationSeconds": 67,
  "timestamp": "2025-10-01T14:32:15Z"
}
```

---

## Event Flow Example

**Successful Generation**:
```
1. engineer:state_changed (idle → analyzing_plan)
2. engineer:reasoning (analyzing phase observations)
3. engineer:state_changed (analyzing_plan → generating_code)
4. engineer:file_started (file 1/17)
5. engineer:reasoning (generating phase progress)
6. engineer:file_completed (file 1/17)
7. engineer:progress (1/17, 5.9%)
8. ... (repeat 4-7 for each file)
9. engineer:state_changed (generating_code → reviewing)
10. engineer:quality_check_started
11. engineer:reasoning (reviewing phase issues/fixes)
12. engineer:quality_check_completed
13. engineer:state_changed (reviewing → complete)
14. engineer:generation_complete
```

**Error During Generation**:
```
1. engineer:state_changed (idle → analyzing_plan)
2. engineer:state_changed (analyzing_plan → generating_code)
3. engineer:file_started (file 1/17)
4. engineer:file_completed (file 1/17)
5. engineer:file_started (file 2/17)
6. engineer:error (generation failed)
7. engineer:state_changed (generating_code → error)
```

**Timeout Handling**:
```
1. engineer:file_started (file 5/17)
2. ... (2 minutes pass)
3. engineer:timeout_prompt (file 5/17)
4. ... (user chooses 'continue')
5. engineer:file_completed (file 5/17)
6. engineer:progress (5/17, 29.4%)
```

---

## Testing Contract

**Contract Tests** (must fail initially):
```rust
// tests/contract/engineer_events_test.rs

#[tokio::test]
async fn test_state_changed_event_emission() {
    let mut rx = subscribe_to_event("engineer:state_changed");
    
    // Trigger state change
    start_generation(test_plan()).await;
    
    let event = rx.recv().await.unwrap();
    assert_eq!(event.previous_state, "idle");
    assert_eq!(event.new_state, "analyzing_plan");
}

#[tokio::test]
async fn test_file_events_emission() {
    let mut started_rx = subscribe_to_event("engineer:file_started");
    let mut completed_rx = subscribe_to_event("engineer:file_completed");
    
    generate_single_file(test_template()).await;
    
    let started = started_rx.recv().await.unwrap();
    assert_eq!(started.path, "test.tsx");
    
    let completed = completed_rx.recv().await.unwrap();
    assert_eq!(completed.path, "test.tsx");
    assert!(completed.lines > 0);
}

#[tokio::test]
async fn test_reasoning_event_stream() {
    let mut rx = subscribe_to_event("engineer:reasoning");
    
    start_generation(test_plan()).await;
    
    let reasoning = rx.recv().await.unwrap();
    assert!(["analyzing", "generating", "reviewing"].contains(&reasoning.phase.as_str()));
}
```

---

**Status**: IPC events contract complete. Ready for quickstart generation.
