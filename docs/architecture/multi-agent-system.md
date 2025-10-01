# Multi-Agent System Architecture

## Overview

Glassflow implements a multi-agent architecture where specialized AI agents collaborate to transform specifications into production-ready code.

## Agent Roles

### 1. Architect Agent (✅ Implemented)

**Purpose**: Analyze specifications and design system architecture

**Responsibilities**:
- Parse user specifications using LLM
- Identify ambiguities and missing information
- Generate clarifying questions
- Design architecture plans with confidence scoring
- Provide component and decision recommendations

**Input**: Natural language specification
**Output**: Architecture plan with components, tech stack, and decisions

**State Machine**:
```
IDLE → ANALYZING → QUESTIONING → DESIGNING → COMPLETE
  ↑        ↓            ↓            ↓           ↓
  ← ← ← ← ERROR ← ← ← ← ← ← ← ← ← ← ← ← ← ← ← ←
```

### 2. Planner Agent (🚧 Planned)

**Purpose**: Break down architecture into actionable tasks

**Responsibilities**:
- Analyze architecture plan
- Identify task dependencies
- Create implementation order
- Estimate complexity and time
- Generate task breakdown

**Input**: Architecture plan
**Output**: Ordered task list with dependencies

### 3. Executor Agent (🚧 Planned)

**Purpose**: Generate code based on tasks

**Responsibilities**:
- Execute tasks in order
- Generate code files
- Apply design patterns
- Ensure code quality
- Handle dependencies

**Input**: Task list
**Output**: Generated code files

### 4. Validator Agent (🚧 Planned)

**Purpose**: Validate and test generated code

**Responsibilities**:
- Run static analysis
- Execute tests
- Check code quality
- Identify issues
- Suggest improvements

**Input**: Generated code
**Output**: Validation report with issues and suggestions

## Agent Communication

### Event-Driven Architecture

Agents communicate through Tauri's event system:

```rust
// Backend emits events
app.emit("architect:state-changed", json!({ ... }));

// Frontend listens
listen('architect:state-changed', (event) => { ... });
```

### IPC Commands

Each agent exposes IPC commands:

```typescript
// Architect commands
invoke('architect_analyze', { spec });
invoke('architect_answer', { answer });
invoke('architect_get_state');
invoke('architect_cancel');
invoke('architect_retry', { spec });
invoke('architect_export_plan', { path });
```

## Data Flow

```
User Input
    ↓
Architect Agent
    ↓ (Architecture Plan)
Planner Agent
    ↓ (Task List)
Executor Agent
    ↓ (Generated Code)
Validator Agent
    ↓ (Validation Report)
Final Output
```

## State Management

### Backend State
- Stored in Rust structs with Mutex for thread safety
- Persisted to `~/.glassflow/architect-state.json`
- Loaded on app startup for session recovery

### Frontend State
- Managed with Zustand stores
- One store per agent
- Real-time updates via event listeners

## Scalability

The architecture is designed to scale:

1. **Agent Independence**: Each agent is self-contained
2. **Clear Interfaces**: Well-defined inputs/outputs
3. **Event-Driven**: Loose coupling between agents
4. **Modular**: Easy to add new agents
5. **Testable**: Each agent can be tested independently

## Future Agents

The system is designed to support additional agents:
- **Optimizer Agent**: Optimize generated code
- **Documenter Agent**: Generate documentation
- **Tester Agent**: Generate test cases
- **Reviewer Agent**: Code review and suggestions
