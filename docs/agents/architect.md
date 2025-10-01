# Architect Agent

## Overview

The Architect Agent is the first agent in the Glassflow pipeline. It analyzes user specifications, asks clarifying questions when needed, and generates detailed architecture plans.

## Status

✅ **Fully Implemented** - Production ready

## Features

### 1. Specification Analysis
- Parses natural language specifications using LLM
- Extracts explicit and implicit requirements
- Identifies technical keywords
- Detects project intent (WebApp, API, CLI, etc.)
- Calculates specification clarity score

### 2. Ambiguity Detection
- Identifies unclear aspects in specifications
- Categorizes ambiguities by impact (High/Medium/Low)
- Suggests clarifying questions
- Prioritizes questions by importance

### 3. Question Generation
- Generates up to 5 most important questions
- Supports multiple question types:
  - Single Choice
  - Multiple Choice
  - Yes/No
  - Free Text
  - Confirmation with Default
- Provides reasoning for each question
- Includes recommended answers when applicable

### 4. Architecture Design
- Generates complete architecture plans
- Recommends tech stack (framework, language, runtime, bundler)
- Suggests architecture patterns (MVC, MVVM, Atomic, etc.)
- Creates component list with purposes and file paths
- Documents architecture decisions with reasoning
- Provides alternatives considered for each decision

### 5. Confidence Scoring
- Calculates overall confidence using weighted formula:
  - Spec Clarity: 25%
  - Technical Feasibility: 25%
  - Architecture Soundness: 20%
  - Completeness: 20%
  - Risk Assessment: 10%
- Provides detailed breakdown of sub-scores

### 6. Session Recovery
- Persists state to `~/.glassflow/architect-state.json`
- Automatically loads saved state on app restart
- Can resume from any state (ANALYZING, QUESTIONING, etc.)
- Preserves all answers and progress

## State Machine

```
IDLE
  ↓ (user enters spec)
ANALYZING
  ↓ (3+ ambiguities found)
QUESTIONING
  ↓ (all questions answered)
DESIGNING
  ↓ (plan generated)
COMPLETE

Any state can transition to:
- ERROR (on failure)
- IDLE (on cancel)
```

## API

### IPC Commands

#### `architect_analyze(spec: string)`
Analyzes a specification and starts the architecture process.

**Flow**:
1. Validates spec is not empty
2. Transitions to ANALYZING
3. Calls LLM to parse specification
4. Decides next state based on ambiguities:
   - 3+ ambiguities → QUESTIONING
   - <3 ambiguities → DESIGNING
5. Persists state

**Returns**: `"Analysis complete"`

#### `architect_answer(answer: string)`
Answers the current question in QUESTIONING state.

**Flow**:
1. Validates state is QUESTIONING
2. Stores answer for current question
3. Advances to next question or transitions to DESIGNING

**Returns**: `void`

#### `architect_get_state()`
Returns the current agent state.

**Returns**: `"idle" | "analyzing" | "questioning" | "designing" | "complete" | "error"`

#### `architect_cancel()`
Cancels the current operation and resets to IDLE.

**Flow**:
1. Transitions to IDLE
2. Clears stored analysis
3. Persists state

**Returns**: `void`

#### `architect_retry(spec: string)`
Retries analysis from ERROR state.

**Flow**:
1. Validates state is ERROR
2. Transitions to IDLE
3. Calls `architect_analyze` with new spec

**Returns**: Same as `architect_analyze`

#### `architect_export_plan(path: string)`
Exports the architecture plan to a JSON file.

**Requirements**: State must be COMPLETE

**Returns**: `void`

### Events

#### `architect:state-changed`
Emitted when state transitions occur.

**Payload**:
```typescript
{
  from: string,
  to: string
}
```

#### `architect:reasoning`
Emitted for reasoning updates during processing.

**Payload**:
```typescript
{
  timestamp: string,
  type: string,
  content: string
}
```

#### `architect:question`
Emitted when a new question is ready.

**Payload**:
```typescript
{
  id: string,
  text: string,
  question_type: string,
  options?: Array<{value, label, description}>,
  recommended_answer?: string,
  reasoning?: string,
  impact: "High" | "Medium" | "Low"
}
```

#### `architect:complete`
Emitted when architecture design is complete.

**Payload**:
```typescript
{
  plan: ArchitecturePlan,
  confidence: ConfidenceBreakdown
}
```

#### `architect:error`
Emitted when an error occurs.

**Payload**:
```typescript
{
  message: string,
  recoverable: boolean
}
```

## Usage Example

```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// Listen to events
listen('architect:state-changed', (event) => {
  console.log(`State: ${event.payload.from} → ${event.payload.to}`);
});

listen('architect:complete', (event) => {
  console.log('Plan:', event.payload.plan);
  console.log('Confidence:', event.payload.confidence.overall);
});

// Start analysis
await invoke('architect_analyze', {
  spec: 'Build a React todo app with TypeScript and Tailwind CSS'
});
```

## File Structure

### Frontend
```
src/agents/architect/
├── components/
│   ├── ArchitectTest.tsx    # Main test component
│   └── QuestionDisplay.tsx  # Question UI component
├── hooks/
│   └── useArchitect.ts      # React hook for architect
└── store/
    └── architectStore.ts    # Zustand store
```

### Backend
```
src-tauri/src/agents/architect/
├── mod.rs           # Module exports
├── analysis.rs      # Specification analysis
├── questions.rs     # Question generation
├── design.rs        # Architecture design
└── confidence.rs    # Confidence calculation
```

## Performance

- **Analysis**: 2-10 seconds (LLM dependent)
- **Question Generation**: 2-5 seconds (LLM dependent)
- **Design**: 5-15 seconds (LLM dependent)
- **Total**: 10-30 seconds for full flow
- **State Persistence**: <10ms

## LLM Integration

Uses OpenRouter API with `x-ai/grok-4-fast:free` model by default.

**Configuration**:
```bash
# Optional: Set custom API key
export OPENROUTER_API_KEY=your_key_here

# Optional: Set custom model
export OPENROUTER_MODEL=anthropic/claude-3-opus
```

## Testing

See test files in:
- `src/__tests__/agents/architect/`
- `src-tauri/tests/contract/test_*_command.rs`

## Future Enhancements

- [ ] Streaming LLM responses for faster perceived performance
- [ ] Caching for similar specifications
- [ ] Multi-language support
- [ ] Custom architecture patterns
- [ ] Integration with external design systems
