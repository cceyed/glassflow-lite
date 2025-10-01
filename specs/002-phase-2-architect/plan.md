
# Implementation Plan: Architect Agent System

**Branch**: `002-phase-2-architect` | **Date**: 2025-09-30 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-phase-2-architect/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from file system structure or context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, `GEMINI.md` for Gemini CLI, `QWEN.md` for Qwen Code or `AGENTS.md` for opencode).
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
The Architect Agent is the first agent in the pipeline that transforms vague user ideas into detailed, unambiguous technical specifications. It implements a 6-state machine (IDLE, ANALYZING, QUESTIONING, DESIGNING, COMPLETE, ERROR) that parses user input, identifies ambiguities, asks clarifying questions (max 7), and produces comprehensive architecture plans with confidence scoring. The agent uses LLM + deterministic parsing for robustness, streams reasoning updates for transparency, and follows glass morphism design patterns. Output includes complete architecture specifications with file structures, component hierarchies, tech stack decisions, and sample code snippets enabling downstream Engineer Agent to implement without clarification.

## Technical Context
**Language/Version**: TypeScript 5.x (frontend), Rust 1.75+ (backend agent logic)  
**Primary Dependencies**: React 18, Tauri (desktop app framework), LLM client library (OpenAI/Anthropic SDK), Zustand (state management), TailwindCSS (styling)  
**Storage**: Local filesystem for state persistence (JSON), no database required  
**Testing**: Vitest (unit/integration), Playwright (e2e), cargo test (Rust backend)  
**Target Platform**: Desktop (macOS/Windows/Linux via Tauri)
**Project Type**: Desktop application (Tauri = Rust backend + React frontend)  
**Performance Goals**: Analysis <5s, design <15s with fast LLM; UI animations 60fps; state transitions <500ms  
**Constraints**: LLM API rate limits (handle with retry logic max 2); offline-capable for state persistence; graceful degradation if LLM unavailable  
**Scale/Scope**: Single-user desktop app; handle specs up to ~10k tokens; support 6 agent states with ~60 functional requirements

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Status**: Constitution file is template-only (not project-specific). Applying general best practices:

- ✅ **Library-First**: Agent logic will be modular Rust library with clear interfaces
- ✅ **Test-First**: TDD approach - write tests before implementation (unit, integration, property tests specified in spec)
- ✅ **Simplicity**: State machine pattern is simple and proven; avoid over-engineering
- ✅ **Observability**: Reasoning stream provides full transparency; all state transitions logged
- ⚠️ **Complexity Note**: LLM integration adds external dependency - justified by core requirement (spec parsing/generation). Fallback to deterministic parsers mitigates risk.

**Initial Check**: PASS (with noted complexity for LLM integration)

## Project Structure

### Documentation (this feature)
```
specs/[###-feature]/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
src-tauri/                          # Rust backend
├── src/
│   ├── agents/
│   │   ├── architect.rs           # Main Architect agent
│   │   ├── state.rs               # State machine implementation
│   │   ├── analysis.rs            # Spec analysis logic
│   │   ├── questions.rs           # Question generation
│   │   ├── design.rs              # Architecture design
│   │   └── confidence.rs          # Confidence calculation
│   ├── llm/
│   │   ├── client.rs              # LLM client trait + impl
│   │   ├── retry.rs               # Retry logic
│   │   └── parsers.rs             # Deterministic fallback parsers
│   ├── models/
│   │   ├── state.rs               # AgentState enum
│   │   ├── analysis.rs            # SpecificationAnalysis
│   │   ├── question.rs            # Question types
│   │   ├── plan.rs                # ArchitecturePlan
│   │   └── reasoning.rs           # ReasoningEntry
│   ├── ipc/
│   │   └── commands.rs            # Tauri IPC handlers
│   ├── persistence/
│   │   └── state_store.rs         # JSON state persistence
│   └── main.rs                    # Tauri app entry
└── tests/
    ├── unit/
    │   ├── state_machine_test.rs
    │   ├── ambiguity_test.rs
    │   ├── questions_test.rs
    │   └── confidence_test.rs
    ├── integration/
    │   └── pipeline_test.rs
    └── fixtures/
        └── test_specs.json

src/                                # React frontend
├── components/
│   ├── architect/
│   │   ├── AgentCard.tsx          # Collapsed agent view
│   │   ├── AgentPanel.tsx         # Expanded agent view
│   │   ├── ReasoningStream.tsx    # Real-time reasoning display
│   │   ├── QuestionDisplay.tsx    # Question UI
│   │   └── PlanViewer.tsx         # Architecture plan display
│   ├── design-system/
│   │   ├── GlassSurface.tsx       # Glass morphism component
│   │   └── LEDIndicator.tsx       # State indicator
│   └── shared/
│       └── ProgressBar.tsx
├── stores/
│   └── architectStore.ts          # Zustand state management
├── hooks/
│   └── useArchitect.ts            # Tauri IPC hooks
└── App.tsx

src/__tests__/                      # Frontend tests
├── components/
│   └── AgentCard.test.tsx
└── integration/
    └── architect-flow.test.tsx
```

**Structure Decision**: Tauri desktop application
- Backend (Rust): State machine, LLM integration, business logic
- Frontend (React): Glass morphism UI, real-time updates
- Clear separation: IPC layer connects frontend ↔ backend
- Tests: Unit (Rust + React), Integration (E2E scenarios)

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context}"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain}"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Entity name, fields, relationships
   - Validation rules from requirements
   - State transitions if applicable

2. **Generate API contracts** from functional requirements:
   - For each user action → endpoint
   - Use standard REST/GraphQL patterns
   - Output OpenAPI/GraphQL schema to `/contracts/`

3. **Generate contract tests** from contracts:
   - One test file per endpoint
   - Assert request/response schemas
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Each story → integration test scenario
   - Quickstart test = story validation steps

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/bash/update-agent-context.sh windsurf`
     **IMPORTANT**: Execute it exactly as specified above. Do not add or remove any arguments.
   - If exists: Add only NEW tech from current plan
   - Preserve manual additions between markers
   - Update recent changes (keep last 3)
   - Keep under 150 lines for token efficiency
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, agent-specific file

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each IPC command → contract test task [P]
- Each entity (AgentState, SpecificationAnalysis, etc.) → model creation task [P]
- Each user story (6 quickstart scenarios) → integration test task
- Implementation tasks to make tests pass
- UI component tasks (AgentCard, AgentPanel, ReasoningStream)

**Ordering Strategy**:
- TDD order: Tests before implementation
- Dependency order: 
  1. Data models (Rust structs)
  2. State machine logic
  3. LLM client + parsers
  4. IPC handlers
  5. UI components
  6. Integration tests
- Mark [P] for parallel execution (independent files)

**Estimated Output**: 40-50 numbered, ordered tasks in tasks.md

**Key Task Groups**:
1. **Backend Models** (10 tasks): AgentState, SpecificationAnalysis, Question, ArchitecturePlan, etc.
2. **State Machine** (8 tasks): State transitions, validation, persistence
3. **LLM Integration** (6 tasks): Client, retry logic, fallback parsers
4. **Analysis Logic** (8 tasks): Ambiguity detection, question generation, confidence calc
5. **IPC Layer** (6 tasks): Tauri commands, event emission
6. **UI Components** (10 tasks): AgentCard, AgentPanel, ReasoningStream, QuestionDisplay
7. **Integration Tests** (6 tasks): One per quickstart scenario

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| LLM external dependency | Core requirement: parse vague specs, generate architecture plans | Deterministic parsing alone cannot handle natural language ambiguity and design reasoning |
| Tauri (Rust + React) | Desktop app with native performance + modern UI | Pure web app lacks offline capability; pure native lacks rapid UI development |


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command) - research.md created
- [x] Phase 1: Design complete (/plan command) - data-model.md, contracts/, quickstart.md created
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [x] Phase 3: Tasks generated (/tasks command) - tasks.md with 64 tasks created
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS (with LLM complexity noted)
- [x] Post-Design Constitution Check: PASS (design follows best practices)
- [x] All NEEDS CLARIFICATION resolved (via research.md)
- [x] Complexity deviations documented (LLM integration justified)

**Artifacts Generated**:
- [x] research.md (10 research decisions)
- [x] data-model.md (10 core entities with validation rules)
- [x] contracts/architect-ipc.json (6 commands, 7 events)
- [x] quickstart.md (6 test scenarios)
- [x] tasks.md (64 tasks across 9 parallel groups, TDD enforced)

---
*Based on Constitution v2.1.1 - See `/memory/constitution.md`*
