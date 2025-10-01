# Orchestrator Documentation

## Overview
The Orchestrator is the lightweight traffic controller for the multi-agent system. It coordinates the flow between all 4 specialist agents without duplicating their complex logic.

## Implementation Status
✅ **COMPLETE & READY**

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  ORCHESTRATOR                       │
│         (Lightweight Coordinator)                   │
│                                                      │
│  • Message Bus                                      │
│  • Pipeline State: Idle → Planning → Building →    │
│                   Validating → Testing → Complete   │
│  • Retry Logic (max 3 per agent)                   │
│  • UI State Aggregation                            │
│  • ~500 lines                                       │
│                                                      │
└──────┬──────────┬──────────┬──────────┬────────────┘
       │          │          │          │
       ↓          ↓          ↓          ↓
   ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐
   │ARCHITECT│ │ENGINEER│ │QUALITY │ │ DEBUG  │
   │        │ │        │ │        │ │        │
   │Complex │ │Complex │ │Complex │ │Complex │
   │States  │ │States  │ │States  │ │States  │
   │Rich    │ │Rich    │ │Rich    │ │Rich    │
   │Logic   │ │Logic   │ │Logic   │ │Logic   │
   └────────┘ └────────┘ └────────┘ └────────┘
```

## Module Structure

```
src-tauri/src/orchestrator/
├── mod.rs           # Main Orchestrator
├── message_bus.rs   # Inter-agent communication
└── pipeline.rs      # Pipeline state management
```

## Core Responsibilities

### 1. Pipeline Management
Coordinates the sequential flow through all agents:

```
User Input
    ↓
🏗️ Architect (Planning) → Architecture Plan
    ↓
⚙️ Engineer (Building) → Code Files
    ↓
✅ Quality (Validating) → Quality Report
    ↓
🐛 Debug (Testing) → Debug Report
    ↓
✅ Production-Ready Code
```

### 2. Message Bus
Simple event system for inter-agent communication:

```rust
pub struct Message {
    pub from: AgentType,
    pub to: AgentType,
    pub payload: MessagePayload,
    pub timestamp: Instant,
}

pub enum MessagePayload {
    StateChanged(String),
    OutputReady(String),
    ErrorOccurred(String),
    ConfidenceUpdated(f32),
    RequestDecision(String),
    ProgressUpdate(f32),
}
```

### 3. Retry Logic
Handles failures with smart retry/rollback decisions:

```rust
// Max 3 retries per agent
const MAX_RETRIES: usize = 3;

// Decision making
match error {
    Recoverable => {
        if retry_count < MAX_RETRIES {
            PipelineDecision::Retry
        } else {
            PipelineDecision::Rollback(previous_phase)
        }
    }
    Critical => PipelineDecision::Abort
}
```

### 4. Phase Management

```rust
pub enum Phase {
    Idle,
    Planning,      // Architect working
    Building,      // Engineer working
    Validating,    // Quality working
    Testing,       // Debug working
    Complete,
    Error(String),
}
```

**Progress Calculation**:
- Idle: 0%
- Planning: 20%
- Building: 40%
- Validating: 60%
- Testing: 80%
- Complete: 100%

## Usage

### Run Full Pipeline

```rust
use crate::orchestrator::Orchestrator;
use crate::llm::client::create_llm_client;

// Create orchestrator
let mut orchestrator = Orchestrator::new();

// Initialize with LLM client
let llm_client = create_llm_client();
orchestrator.initialize(llm_client).await;

// Run the full pipeline
match orchestrator.run_pipeline(user_prompt).await {
    Ok(result) => {
        println!("Pipeline complete!");
        println!("Overall confidence: {:.1}%", result.overall_confidence);
        println!("Phases completed: {}", result.phases_completed);
        println!("Total retries: {}", result.total_retries);
    }
    Err(e) => {
        eprintln!("Pipeline failed: {}", e);
    }
}
```

### IPC Commands

```typescript
// Run full pipeline
await invoke('orchestrator_run_pipeline', {
  prompt: 'Build a todo app with React and TypeScript'
});

// Get current status
const status = await invoke('orchestrator_get_status');
// Returns: { current_phase, progress, is_running }

// Reset orchestrator
await invoke('orchestrator_reset');
```

## Pipeline Result

```rust
pub struct PipelineResult {
    pub code_output: CodeOutput,
    pub quality_report: QualityReport,
    pub debug_report: DebugReport,
    pub overall_confidence: f32,
    pub phases_completed: usize,
    pub total_retries: usize,
}
```

## Error Handling

### Example: Debug Finds Critical Bug

**Without Orchestrator (chaos)**:
```
Debug: "Found critical bug, need Engineer to regenerate"
Debug → Engineer: "Here's the issue"
Engineer: "Ok, regenerating..."
Quality: "Wait, what's happening?"
UI: "Which agent is active???"
```

**With Orchestrator (clean)**:
```
Debug → Orchestrator: Message::ErrorOccurred(critical_bug)
Orchestrator: Analyzes error, decides to rollback
Orchestrator → Engineer: Command::Regenerate(debug_feedback)
Orchestrator → UI: Update(Phase::Building, retry_count=1)
Orchestrator → Quality: Command::Pause
All agents in sync, user sees clear state
```

## Decision Making

The Orchestrator makes intelligent decisions based on error types:

1. **Clarification Needed** → Rollback to Planning (Architect)
2. **Code Generation Failed** → Retry Building (max 3 times)
3. **Quality Issues** → Retry Validating or rollback to Building
4. **Runtime Bugs** → Retry Testing or rollback to Building
5. **Critical Errors** → Abort immediately

## Confidence Calculation

Overall confidence is a weighted average:
- **Quality Report**: 40%
- **Debug Report**: 60% (runtime is more important)

```rust
overall_confidence = (quality.confidence * 0.4) + (debug.confidence * 0.6)
```

## Key Design Principles

### DO ✅
- Keep it lightweight (~500 lines)
- Simple message passing
- Basic retry logic (max 3)
- UI state aggregation
- Clear phase transitions

### DON'T ❌
- Duplicate agent logic
- Complex state machines
- Heavy processing
- Business logic
- Direct agent-to-agent communication

## Separation of Concerns

| Component | Role | Complexity |
|-----------|------|------------|
| **Architect** | Design blueprints | High ✅ |
| **Engineer** | Generate code | High ✅ |
| **Quality** | Validate standards | High ✅ |
| **Debug** | Test runtime | High ✅ |
| **Orchestrator** | Coordinate flow | **LOW** ✅ |

## Communication Patterns

```rust
// Agent → Orchestrator (push updates)
agent.emit(Message::StateChanged(new_state));

// Orchestrator → Agent (send commands)
orchestrator.send_to(agent, Command::Start(input));

// Agent ↔ Agent (through orchestrator)
quality_agent.request_from(engineer_agent, Request::GetCode);
// ↓ becomes ↓
orchestrator.route(quality_agent, engineer_agent, request);
```

## Future Enhancements

1. **Parallel Execution**: Run Quality and Debug in parallel when safe
2. **Smart Caching**: Cache intermediate results
3. **Rollback Points**: Save state at each phase for quick rollback
4. **Custom Pipelines**: Allow users to configure agent order
5. **Pipeline Templates**: Pre-configured flows for common tasks

## Integration with UI

The Orchestrator provides a single source of truth for UI state:

```typescript
// UI polls for status
const status = await invoke('orchestrator_get_status');

// Display progress bar
<ProgressBar value={status.progress} />

// Show current phase
<PhaseIndicator phase={status.current_phase} />

// Enable/disable controls
<Button disabled={status.is_running}>Run Pipeline</Button>
```

## Testing

Test the full pipeline via UI:
1. Select "Pipeline" tab
2. Enter project specification
3. Click "Run Full Pipeline"
4. Watch all 4 agents execute in sequence
5. View final results

## Summary

The Orchestrator is the **glue** that holds the multi-agent system together:
- ✅ Lightweight and focused
- ✅ Coordinates without duplicating
- ✅ Handles errors intelligently
- ✅ Provides clear UI state
- ✅ Enables retry/rollback logic

**The complexity is where it belongs:**
- 4 Specialist Agents: Rich, complex, domain experts ✅
- 1 Coordinator: Lean, simple, traffic controller ✅
