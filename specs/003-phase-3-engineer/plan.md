
# Implementation Plan: Engineer Agent System

**Branch**: `003-phase-3-engineer` | **Date**: 2025-10-01 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-phase-3-engineer/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → ✓ Loaded successfully
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → ✓ All clarifications resolved (5 questions answered)
   → ✓ Project Type: Desktop application (Tauri)
3. Fill the Constitution Check section based on the content of the constitution document.
   → ✓ Constitution template reviewed
4. Evaluate Constitution Check section below
   → ✓ No violations, following best practices
   → ✓ Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → ✓ Ready to generate
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file
   → ✓ Ready to generate
7. Re-evaluate Constitution Check section
   → Will verify after Phase 1 design
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
   → ✓ Approach defined below
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 8. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
The Engineer Agent is the second agent in the pipeline that takes architecture plans from the Architect Agent and generates clean, functional code. It implements a 6-state machine (IDLE, ANALYZING_PLAN, GENERATING_CODE, REVIEWING, COMPLETE, ERROR) that parses architecture plans, generates code files with proper types/imports/documentation, performs self-review with auto-fix capabilities, and produces compilable code with confidence scoring. The agent supports concurrent generation for independent files, user-configurable output directories, timeout handling (2min prompt), and validates external imports against architecture plan dependencies. Output includes all generated files, quality reports, and file-by-file breakdowns enabling downstream Quality Agent to validate.

## Technical Context
**Language/Version**: TypeScript 5.x (frontend), Rust 1.75+ (backend agent logic)  
**Primary Dependencies**: React 18, Tauri (desktop app framework), LLM client library (OpenAI/Anthropic SDK), Zustand (state management), TailwindCSS (styling), TypeScript compiler API (syntax validation), ESLint/Prettier (code quality)  
**Storage**: User-configurable output directory (default: project root), local filesystem for progress persistence (JSON)  
**Testing**: Vitest (unit/integration), Playwright (e2e), cargo test (Rust backend)  
**Target Platform**: Desktop (macOS/Windows/Linux via Tauri)
**Project Type**: Desktop application (Tauri = Rust backend + React frontend)  
**Performance Goals**: File generation <2min per file (with timeout prompt); concurrent generation for independent files; UI animations 60fps; state transitions <500ms  
**Constraints**: LLM API rate limits (handle with retry logic); preserve only completed files on error; validate imports against architecture plan dependencies only (no runtime package.json checks)  
**Scale/Scope**: Single-user desktop app; handle architecture plans with up to ~50 files; support 6 agent states with ~68 functional requirements; concurrent generation for independent files

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Status**: Constitution file is template-only (not project-specific). Applying general best practices:

- ✅ **Library-First**: Agent logic will be modular Rust library with clear interfaces (code generator, quality checker, confidence calculator)
- ✅ **Test-First**: TDD approach - write tests before implementation (unit, integration, property tests specified in spec)
- ✅ **Simplicity**: State machine pattern is simple and proven; reuses patterns from Architect Agent
- ✅ **Observability**: Reasoning stream provides full transparency; all state transitions and generation progress logged
- ⚠️ **Complexity Note**: LLM integration for code generation adds external dependency - justified by core requirement (code generation from templates). Concurrent generation adds complexity but justified by performance requirements.

**Initial Check**: PASS (with noted complexity for LLM integration and concurrency)

## Project Structure

### Documentation (this feature)
```
specs/003-phase-3-engineer/
├── spec.md              # Feature specification (completed)
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
│   │   ├── engineer.rs            # Main Engineer agent
│   │   ├── state.rs               # State machine (reuse from Architect)
│   │   ├── plan_analysis.rs      # Architecture plan parsing
│   │   ├── code_generator.rs     # Code generation logic
│   │   ├── quality_checker.rs    # Quality validation
│   │   ├── auto_fixer.rs          # Auto-fix minor issues
│   │   └── confidence.rs          # Confidence calculation (reuse/extend)
│   ├── llm/
│   │   ├── client.rs              # LLM client (reuse from Architect)
│   │   ├── retry.rs               # Retry logic (reuse)
│   │   └── streaming.rs           # Streaming code generation
│   ├── models/
│   │   ├── engineer_state.rs     # EngineerState enum
│   │   ├── architecture_plan.rs  # ArchitecturePlan (from Architect)
│   │   ├── generated_file.rs     # GeneratedFile
│   │   ├── quality_check.rs      # CodeQualityCheck, QualityIssue
│   │   ├── code_output.rs        # CodeOutput
│   │   └── reasoning.rs           # ReasoningEntry (reuse)
│   ├── codegen/
│   │   ├── template_engine.rs    # Template-based generation
│   │   ├── import_resolver.rs    # Import resolution
│   │   ├── type_checker.rs       # Type validation
│   │   └── syntax_validator.rs   # Syntax checking
│   ├── concurrent/
│   │   ├── file_scheduler.rs     # Dependency-based scheduling
│   │   └── parallel_gen.rs       # Concurrent generation
│   ├── ipc/
│   │   └── engineer_commands.rs  # Tauri IPC handlers for Engineer
│   ├── persistence/
│   │   ├── progress_store.rs     # Generation progress persistence
│   │   └── output_writer.rs      # File writing to configurable directory
│   └── main.rs                    # Tauri app entry (shared)
└── tests/
    ├── unit/
    │   ├── plan_analysis_test.rs
    │   ├── code_generator_test.rs
    │   ├── quality_checker_test.rs
    │   ├── confidence_test.rs
    │   └── concurrent_test.rs
    ├── integration/
    │   ├── full_generation_flow_test.rs
    │   ├── self_review_cycle_test.rs
    │   └── error_recovery_test.rs
    └── property/
        ├── random_plan_generator.rs
        └── code_validation_invariants.rs

src/                                # React frontend
├── components/
│   ├── engineer/
│   │   ├── EngineerCard.tsx       # Engineer agent card
│   │   ├── EngineerPanel.tsx      # Expanded panel with reasoning
│   │   ├── CodePreview.tsx        # Real-time code preview
│   │   ├── FileProgress.tsx       # File-by-file progress
│   │   └── QualityReport.tsx      # Quality check results
│   └── design-system/             # Existing design system (reuse)
├── hooks/
│   └── useEngineer.ts             # Engineer agent state hook
└── types/
    └── engineer.ts                # TypeScript types for Engineer

src/__tests__/
├── components/
│   └── engineer/
│       ├── EngineerCard.test.tsx
│       ├── EngineerPanel.test.tsx
│       └── CodePreview.test.tsx
└── e2e/
    └── engineer-flow.spec.ts      # End-to-end generation flow
```

**Structure Decision**: Desktop application using Tauri architecture (Rust backend + React frontend). Engineer Agent follows the same pattern as Architect Agent, with backend logic in `src-tauri/src/agents/engineer.rs` and frontend UI in `src/components/engineer/`. Code generation logic is modularized into `codegen/` directory. Concurrent generation support added in `concurrent/` directory. Reuses existing infrastructure (LLM client, state machine, design system) from Architect Agent implementation.

## Phase 0: Outline & Research

**Status**: All clarifications resolved via `/clarify` workflow. No unknowns remain in Technical Context.

**Research Topics**:
1. **Concurrent Code Generation Patterns**
   - Decision: Use Tokio async runtime with semaphore-based concurrency control
   - Rationale: Tokio provides robust async/await support in Rust; semaphore prevents overwhelming LLM API
   - Alternatives: Thread pool (more complex), sequential only (slower)

2. **Code Quality Validation Approaches**
   - Decision: Use TypeScript Compiler API for syntax/type checking + ESLint for style
   - Rationale: Official TypeScript tooling ensures accuracy; ESLint widely adopted
   - Alternatives: Custom parser (reinventing wheel), regex-based (fragile)

3. **Timeout Handling Strategy**
   - Decision: Use Tokio timeout with user prompt on expiration
   - Rationale: Non-blocking, allows user control, prevents infinite hangs
   - Alternatives: Hard abort (poor UX), no timeout (risk of hangs)

4. **File Output Management**
   - Decision: Atomic writes with temp files + rename, configurable output directory
   - Rationale: Prevents partial file corruption; user control over destination
   - Alternatives: Direct writes (unsafe), fixed directory (inflexible)

5. **Import Resolution Strategy**
   - Decision: Validate against architecture plan's dependency list only
   - Rationale: Architecture plan is source of truth; avoids filesystem coupling
   - Alternatives: Check package.json (runtime dependency), filesystem scan (slow)

**Output**: research.md (generated below)

## Phase 1: Design & Contracts
*Prerequisites: research.md complete ✓*

### 1. Data Model (`data-model.md`)
**Entities extracted from spec (9 key entities)**:
- EngineerState (state machine)
- ArchitecturePlan (input from Architect)
- FileTemplate (file specification)
- GeneratedFile (output file)
- CodeQualityCheck (validation results)
- QualityIssue (individual issue)
- ConfidenceBreakdown (metrics)
- ReasoningEntry (transparency log)
- CodeOutput (final output bundle)

**State transitions**: IDLE → ANALYZING_PLAN → GENERATING_CODE → REVIEWING → COMPLETE/ERROR

### 2. IPC Contracts (`contracts/`)
**Tauri commands** (Rust → Frontend):
- `start_code_generation(plan: ArchitecturePlan) → Result<()>`
- `get_engineer_state() → EngineerState`
- `get_generation_progress() → GenerationProgress`
- `cancel_generation() → Result<()>`
- `retry_generation(modified_plan: Option<ArchitecturePlan>) → Result<()>`
- `get_quality_report() → QualityReport`
- `export_generated_code(directory: PathBuf) → Result<()>`

**Events** (Frontend ← Rust):
- `engineer:state_changed` → EngineerState
- `engineer:file_started` → { path: string, index: number, total: number }
- `engineer:file_completed` → { path: string, lines: number, confidence: number }
- `engineer:reasoning` → ReasoningEntry
- `engineer:progress` → { completed: number, total: number, percentage: number }
- `engineer:error` → { message: string, recoverable: boolean }

### 3. Contract Tests
**Test files** (must fail initially):
- `tests/contract/engineer_commands_test.rs` - IPC command contracts
- `tests/contract/engineer_events_test.rs` - Event emission contracts
- `tests/contract/quality_validation_test.rs` - Quality check contracts

### 4. Integration Test Scenarios
From acceptance scenarios in spec:
- **Scenario 1**: Receive plan → Analyze → Generate files → Complete
- **Scenario 2**: Generate 17 files with progress tracking
- **Scenario 3**: Self-review detects issues → Auto-fix → Re-validate
- **Scenario 4**: Generation error → ERROR state → Retry with modified plan
- **Scenario 5**: Timeout handling → User prompt → Continue/Cancel/Skip

### 5. Quickstart Test (`quickstart.md`)
End-to-end validation:
1. Load sample architecture plan (React todo app, 17 files)
2. Start Engineer Agent
3. Verify all files generated with correct structure
4. Run TypeScript compiler on output (must pass)
5. Verify confidence >85%
6. Check quality report (no critical issues)

### 6. Agent Context Update
Running update script to add Engineer Agent context...

**Output**: data-model.md ✓, contracts/ ✓, failing tests ✓, quickstart.md ✓, agent context updated ✓

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
1. Load `.specify/templates/tasks-template.md` as base
2. Generate tasks from Phase 1 artifacts:
   - **From data-model.md**: Create Rust structs/enums for 9 entities
   - **From contracts/**: Create IPC command handlers and event emitters
   - **From quickstart.md**: Create integration test scenarios
3. Follow TDD approach: Tests before implementation
4. Group by subsystem: Models → Core Logic → IPC → UI → Integration

**Task Categories**:
- **Models** (9 tasks): Create Rust data structures [P]
- **Contract Tests** (15 tasks): IPC command/event tests [P]
- **Core Logic** (12 tasks): Code generator, quality checker, confidence calculator
- **Concurrent Generation** (3 tasks): Tokio-based parallel generation
- **IPC Integration** (8 tasks): Tauri command handlers
- **UI Components** (5 tasks): React components for Engineer Agent
- **Integration Tests** (5 tasks): End-to-end scenarios from quickstart
- **Documentation** (2 tasks): API docs, user guide

**Ordering Strategy**:
1. **Phase 1: Models & Contracts** (TDD)
   - Create model structs [P]
   - Write contract tests (must fail) [P]
   - Implement IPC contracts to pass tests

2. **Phase 2: Core Logic** (TDD)
   - Write unit tests for code generator [P]
   - Implement code generator
   - Write unit tests for quality checker [P]
   - Implement quality checker
   - Write unit tests for confidence calculator [P]
   - Implement confidence calculator

3. **Phase 3: Concurrent Generation**
   - Implement Tokio-based file scheduler
   - Implement parallel generation with semaphore
   - Add timeout handling

4. **Phase 4: IPC Integration**
   - Implement Tauri command handlers
   - Implement event emission
   - Wire up state machine

5. **Phase 5: UI Components**
   - Create EngineerCard component
   - Create EngineerPanel component
   - Create CodePreview component
   - Create QualityReport component

6. **Phase 6: Integration & Validation**
   - Run integration tests from quickstart
   - Performance benchmarking
   - End-to-end validation

**Parallelization**:
- Mark [P] for independent tasks (models, contract tests, unit tests)
- Sequential for dependent tasks (implementation after tests)

**Estimated Output**: 55-60 numbered, ordered tasks in tasks.md

**Dependencies**:
- Reuse existing infrastructure: LLM client, state machine, design system
- Extend Architect Agent patterns: reasoning stream, confidence calculation
- New subsystems: Code generator, quality checker, concurrent scheduler

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
| LLM Integration | Core requirement: code generation from templates | Manual code writing defeats purpose of automation |
| Concurrent Generation | Performance requirement: 3-5x faster for large projects | Sequential generation too slow (>5min for 50 files) |
| TypeScript Compiler API | Accurate syntax/type validation required | Custom parser would be fragile and incomplete |

**Justification**: All complexity is essential to meet functional requirements. LLM integration is the core value proposition. Concurrent generation is necessary for acceptable performance. TypeScript Compiler API ensures correctness.

---

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command) ✓
- [x] Phase 1: Design complete (/plan command) ✓
- [x] Phase 2: Task planning complete (/plan command - describe approach only) ✓
- [ ] Phase 3: Tasks generated (/tasks command) - NEXT STEP
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS ✓
- [x] Post-Design Constitution Check: PASS ✓
- [x] All NEEDS CLARIFICATION resolved ✓
- [x] Complexity deviations documented ✓

**Artifacts Generated**:
- [x] research.md (5 technical decisions documented)
- [x] data-model.md (9 entities with relationships)
- [x] contracts/ipc-commands.md (9 commands specified)
- [x] contracts/ipc-events.md (10 events specified)
- [x] quickstart.md (end-to-end validation scenario)

**Ready for /tasks command**: ✓

---
*Based on Constitution v2.1.1 - See `/memory/constitution.md`*
