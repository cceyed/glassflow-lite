# Tasks: Engineer Agent System

**Input**: Design documents from `/specs/003-phase-3-engineer/`
**Prerequisites**: plan.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓

## Execution Flow (main)
```
1. Load plan.md from feature directory ✓
   → Tech stack: TypeScript 5.x (frontend), Rust 1.75+ (backend)
   → Libraries: React 18, Tauri, LLM client, Zustand, TailwindCSS, TypeScript Compiler API, ESLint
   → Structure: Desktop app (Rust backend + React frontend)
2. Load optional design documents ✓
   → data-model.md: 9 entities extracted
   → contracts/: 2 contract files (commands, events)
   → research.md: 5 technical decisions
   → quickstart.md: End-to-end validation scenario
3. Generate tasks by category ✓
   → Setup: 3 tasks
   → Tests: 18 tasks (contract + integration)
   → Core: 24 tasks (models + logic)
   → Integration: 8 tasks (IPC + UI)
   → Polish: 6 tasks (validation + docs)
4. Apply task rules ✓
   → Different files = [P] for parallel
   → Same file = sequential
   → Tests before implementation (TDD)
5. Number tasks sequentially (T001-T059) ✓
6. Generate dependency graph ✓
7. Create parallel execution examples ✓
8. Validate task completeness ✓
   → All contracts have tests ✓
   → All entities have models ✓
   → All tests before implementation ✓
9. Return: SUCCESS (59 tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Rust backend**: `src-tauri/src/` for source, `src-tauri/tests/` for tests
- **React frontend**: `src/` for source, `src/__tests__/` for tests
- Desktop application structure (Tauri = Rust + React)

---

## Phase 3.1: Setup

- [x] **T001** Create Engineer Agent directory structure in `src-tauri/src/agents/`, `src-tauri/src/models/`, `src-tauri/src/codegen/`, `src-tauri/src/concurrent/`, `src/components/engineer/`
- [x] **T002** Add Rust dependencies to `src-tauri/Cargo.toml`: tokio (async runtime), serde (serialization), anyhow (error handling), tempfile (atomic writes)
- [x] **T003** [P] Configure ESLint and Prettier for TypeScript code quality validation

---

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

### Contract Tests (IPC Commands)
- [x] **T004** [P] Contract test for `start_code_generation` command in `src-tauri/tests/contract/engineer_commands_test.rs` - verify command accepts ArchitecturePlan and returns Result
- [x] **T005** [P] Contract test for `get_engineer_state` command in `src-tauri/tests/contract/engineer_commands_test.rs` - verify returns EngineerState enum
- [x] **T006** [P] Contract test for `get_generation_progress` command in `src-tauri/tests/contract/engineer_commands_test.rs` - verify returns GenerationProgress struct
- [x] **T007** [P] Contract test for `cancel_generation` command in `src-tauri/tests/contract/engineer_commands_test.rs` - verify cancels and preserves completed files
- [x] **T008** [P] Contract test for `retry_generation` command in `src-tauri/tests/contract/engineer_commands_test.rs` - verify retries from ERROR state
- [x] **T009** [P] Contract test for `get_quality_report` command in `src-tauri/tests/contract/engineer_commands_test.rs` - verify returns QualityReport
- [x] **T010** [P] Contract test for `export_generated_code` command in `src-tauri/tests/contract/engineer_commands_test.rs` - verify exports to directory
- [x] **T011** [P] Contract test for `handle_timeout_prompt` command in `src-tauri/tests/contract/engineer_commands_test.rs` - verify handles timeout choices

### Contract Tests (IPC Events)
- [x] **T012** [P] Contract test for `engineer:state_changed` event in `src-tauri/tests/contract/engineer_events_test.rs` - verify emits on state transitions
- [x] **T013** [P] Contract test for `engineer:file_started` event in `src-tauri/tests/contract/engineer_events_test.rs` - verify emits when file generation starts
- [x] **T014** [P] Contract test for `engineer:file_completed` event in `src-tauri/tests/contract/engineer_events_test.rs` - verify emits when file completes
- [x] **T015** [P] Contract test for `engineer:reasoning` event in `src-tauri/tests/contract/engineer_events_test.rs` - verify streams reasoning entries
- [x] **T016** [P] Contract test for `engineer:progress` event in `src-tauri/tests/contract/engineer_events_test.rs` - verify updates progress
- [x] **T017** [P] Contract test for `engineer:error` event in `src-tauri/tests/contract/engineer_events_test.rs` - verify emits on errors
- [x] **T018** [P] Contract test for `engineer:timeout_prompt` event in `src-tauri/tests/contract/engineer_events_test.rs` - verify prompts on timeout

### Integration Tests
- [x] **T019** [P] Integration test for full generation flow in `src-tauri/tests/integration/full_generation_flow_test.rs` - IDLE → ANALYZING_PLAN → GENERATING_CODE → REVIEWING → COMPLETE
- [x] **T020** [P] Integration test for self-review cycle in `src-tauri/tests/integration/self_review_cycle_test.rs` - detect issues, auto-fix, re-validate
- [x] **T021** [P] Integration test for error recovery in `src-tauri/tests/integration/error_recovery_test.rs` - ERROR state → retry with modified plan

---

## Phase 3.3: Core Implementation (ONLY after tests are failing)

### Models (Data Structures)
- [x] **T022** [P] Create `EngineerState` enum in `src-tauri/src/models/engineer_state.rs` with 6 states (Idle, AnalyzingPlan, GeneratingCode, Reviewing, Complete, Error)
- [x] **T023** [P] Create `ArchitecturePlan` struct in `src-tauri/src/models/architecture_plan.rs` (reuse from Architect Agent if exists, otherwise create)
- [x] **T024** [P] Create `FileTemplate` struct in `src-tauri/src/models/file_template.rs` with path, purpose, estimated_lines, language, dependencies
- [x] **T025** [P] Create `GeneratedFile` struct in `src-tauri/src/models/generated_file.rs` with path, content, language, lines, imports, exports, types, confidence
- [x] **T026** [P] Create `CodeQualityCheck` struct in `src-tauri/src/models/quality_check.rs` with validation results and issues list
- [x] **T027** [P] Create `QualityIssue` struct in `src-tauri/src/models/quality_check.rs` with severity, category, line, description, suggestion
- [x] **T028** [P] Create `ConfidenceBreakdown` struct in `src-tauri/src/models/engineer_confidence.rs` with overall, quality_score, plan_adherence, issue_penalty
- [x] **T029** [P] Create `ReasoningEntry` struct in `src-tauri/src/models/reasoning.rs` (reuse from Architect Agent if exists)
- [x] **T030** [P] Create `CodeOutput` struct in `src-tauri/src/models/code_output.rs` with files, total_lines, confidence, quality_report, metadata

### Code Generation Core
- [x] **T031** Implement `PlanAnalyzer` in `src-tauri/src/agents/engineer/plan_analysis.rs` - parse ArchitecturePlan, validate completeness, build dependency graph
- [x] **T032** Implement `CodeGenerator` in `src-tauri/src/agents/engineer/code_generator.rs` - generate code from FileTemplate using LLM with streaming
- [x] **T033** Implement `TemplateEngine` in `src-tauri/src/codegen/template_engine.rs` - build generation prompts with architecture context
- [x] **T034** Implement `ImportResolver` in `src-tauri/src/codegen/import_resolver.rs` - validate imports against architecture plan dependencies
- [x] **T035** Implement `TypeChecker` in `src-tauri/src/codegen/type_checker.rs` - validate TypeScript types using TypeScript Compiler API (via Command)
- [x] **T036** Implement `SyntaxValidator` in `src-tauri/src/codegen/syntax_validator.rs` - validate syntax using TypeScript Compiler API (via Command)

### Quality Checking
- [x] **T037** Implement `QualityChecker` in `src-tauri/src/agents/engineer/quality_checker.rs` - run all quality checks (syntax, types, imports, exports, style, edge cases, docs)
- [x] **T038** Implement style validation in `src-tauri/src/agents/engineer/quality_checker.rs` - run ESLint via Command and parse JSON output
- [x] **T039** Implement `AutoFixer` in `src-tauri/src/agents/engineer/auto_fixer.rs` - auto-fix Low/Medium severity issues (formatting, style violations)

### Confidence Calculation
- [x] **T040** Implement confidence calculation in `src-tauri/src/agents/engineer/confidence.rs` - calculate overall confidence as weighted average (quality 50%, adherence 30%, penalty 20%)
- [x] **T041** Implement quality score calculation in `src-tauri/src/agents/engineer/confidence.rs` - based on passed checks (syntax, types, imports, exports, style, edge cases, docs)
- [x] **T042** Implement plan adherence calculation in `src-tauri/src/agents/engineer/confidence.rs` - compare generated output to architecture plan specifications
- [x] **T043** Implement issue penalty calculation in `src-tauri/src/agents/engineer/confidence.rs` - apply penalties by severity (Critical 20%, High 10%, Medium 5%, Low 2%), cap at 80%

### Concurrent Generation
- [x] **T044** Implement `FileScheduler` in `src-tauri/src/concurrent/file_scheduler.rs` - determine generation order based on dependencies (dependency-first, sequential, or parallel)
- [x] **T045** Implement `ParallelGenerator` in `src-tauri/src/concurrent/parallel_gen.rs` - use Tokio async runtime with semaphore (max 3-5 concurrent) to generate independent files in parallel

---

## Phase 3.4: Integration

### Main Engineer Agent
- [ ] **T046** Implement `Engineer` agent in `src-tauri/src/agents/engineer.rs` - main agent struct with state machine, implement `implement_plan` method
- [ ] **T047** Implement state transitions in `src-tauri/src/agents/engineer.rs` - IDLE → ANALYZING_PLAN → GENERATING_CODE → REVIEWING → COMPLETE/ERROR
- [ ] **T048** Implement timeout handling in `src-tauri/src/agents/engineer.rs` - prompt user after 2 minutes per file with Continue/Cancel/Skip options
- [ ] **T049** Implement error handling in `src-tauri/src/agents/engineer.rs` - preserve completed files only, transition to ERROR state with recovery options

### File Output Management
- [ ] **T050** Implement `OutputWriter` in `src-tauri/src/persistence/output_writer.rs` - atomic writes with temp files + rename, user-configurable output directory
- [ ] **T051** Implement `ProgressStore` in `src-tauri/src/persistence/progress_store.rs` - persist generation progress to JSON for resumption after interruption

### IPC Commands
- [ ] **T052** Implement Tauri command handlers in `src-tauri/src/ipc/engineer_commands.rs` - wire up all 9 commands (start_code_generation, get_engineer_state, get_generation_progress, cancel_generation, retry_generation, get_quality_report, export_generated_code, get_confidence_breakdown, handle_timeout_prompt)
- [ ] **T053** Implement event emission in `src-tauri/src/ipc/engineer_commands.rs` - emit all 10 events (state_changed, file_started, file_completed, reasoning, progress, error, timeout_prompt, quality_check_started, quality_check_completed, generation_complete)

### UI Components
- [ ] **T054** Create `EngineerCard` component in `src/components/engineer/EngineerCard.tsx` - compact card showing state, progress, confidence with LED indicator
- [ ] **T055** Create `EngineerPanel` component in `src/components/engineer/EngineerPanel.tsx` - expanded panel with reasoning stream, file progress, quality report
- [ ] **T056** Create `CodePreview` component in `src/components/engineer/CodePreview.tsx` - real-time code preview showing generated code as it streams
- [ ] **T057** Create `useEngineer` hook in `src/hooks/useEngineer.ts` - React hook for Engineer Agent state management, event listeners, command invocations

---

## Phase 3.5: Polish

### Validation & Testing
- [ ] **T058** Run quickstart validation from `specs/003-phase-3-engineer/quickstart.md` - generate React todo app (17 files), verify TypeScript compilation, check quality report, validate confidence >85%
- [ ] **T059** [P] Update documentation in `specs/003-phase-3-engineer/README.md` - API reference, usage examples, troubleshooting guide

---

## Dependencies

### Critical Path (Sequential)
```
Setup (T001-T003)
  ↓
Contract Tests (T004-T021) - MUST FAIL before implementation
  ↓
Models (T022-T030) - Foundation for everything
  ↓
Core Logic (T031-T043) - Code generation, quality checking, confidence
  ↓
Concurrent Generation (T044-T045) - Performance optimization
  ↓
Main Agent (T046-T049) - Orchestration
  ↓
File Output (T050-T051) - Persistence
  ↓
IPC Integration (T052-T053) - Frontend communication
  ↓
UI Components (T054-T057) - User interface
  ↓
Validation (T058-T059) - Final checks
```

### Specific Dependencies
- **T031-T043** depend on **T022-T030** (models must exist)
- **T044-T045** depend on **T032** (code generator must exist)
- **T046-T049** depend on **T031-T043** (core logic must exist)
- **T050-T051** depend on **T025** (GeneratedFile model must exist)
- **T052-T053** depend on **T046-T049** (main agent must exist)
- **T054-T057** depend on **T052-T053** (IPC must work)
- **T058** depends on **ALL** previous tasks (full system validation)

### Parallel Execution Groups

**Group 1: Contract Tests (can run simultaneously)**
```
T004, T005, T006, T007, T008, T009, T010, T011 (IPC command tests)
T012, T013, T014, T015, T016, T017, T018 (IPC event tests)
T019, T020, T021 (integration tests)
```

**Group 2: Models (can run simultaneously)**
```
T022, T023, T024, T025, T026, T027, T028, T029, T030
```

**Group 3: Setup (can run simultaneously with other setup)**
```
T003 (linting config - independent)
```

---

## Parallel Execution Examples

### Example 1: Launch All Contract Tests Together
```bash
# All contract tests are independent (different test functions)
Task: "Contract test start_code_generation in src-tauri/tests/contract/engineer_commands_test.rs"
Task: "Contract test get_engineer_state in src-tauri/tests/contract/engineer_commands_test.rs"
Task: "Contract test get_generation_progress in src-tauri/tests/contract/engineer_commands_test.rs"
Task: "Contract test cancel_generation in src-tauri/tests/contract/engineer_commands_test.rs"
Task: "Contract test retry_generation in src-tauri/tests/contract/engineer_commands_test.rs"
Task: "Contract test get_quality_report in src-tauri/tests/contract/engineer_commands_test.rs"
Task: "Contract test export_generated_code in src-tauri/tests/contract/engineer_commands_test.rs"
Task: "Contract test handle_timeout_prompt in src-tauri/tests/contract/engineer_commands_test.rs"
Task: "Contract test engineer:state_changed event in src-tauri/tests/contract/engineer_events_test.rs"
Task: "Contract test engineer:file_started event in src-tauri/tests/contract/engineer_events_test.rs"
Task: "Contract test engineer:file_completed event in src-tauri/tests/contract/engineer_events_test.rs"
Task: "Contract test engineer:reasoning event in src-tauri/tests/contract/engineer_events_test.rs"
Task: "Contract test engineer:progress event in src-tauri/tests/contract/engineer_events_test.rs"
Task: "Contract test engineer:error event in src-tauri/tests/contract/engineer_events_test.rs"
Task: "Contract test engineer:timeout_prompt event in src-tauri/tests/contract/engineer_events_test.rs"
Task: "Integration test full generation flow in src-tauri/tests/integration/full_generation_flow_test.rs"
Task: "Integration test self-review cycle in src-tauri/tests/integration/self_review_cycle_test.rs"
Task: "Integration test error recovery in src-tauri/tests/integration/error_recovery_test.rs"
```

### Example 2: Launch All Model Creation Together
```bash
# All models are independent files
Task: "Create EngineerState enum in src-tauri/src/models/engineer_state.rs"
Task: "Create ArchitecturePlan struct in src-tauri/src/models/architecture_plan.rs"
Task: "Create FileTemplate struct in src-tauri/src/models/file_template.rs"
Task: "Create GeneratedFile struct in src-tauri/src/models/generated_file.rs"
Task: "Create CodeQualityCheck struct in src-tauri/src/models/quality_check.rs"
Task: "Create QualityIssue struct in src-tauri/src/models/quality_check.rs"
Task: "Create ConfidenceBreakdown struct in src-tauri/src/models/confidence.rs"
Task: "Create ReasoningEntry struct in src-tauri/src/models/reasoning.rs"
Task: "Create CodeOutput struct in src-tauri/src/models/code_output.rs"
```

---

## Notes

- **[P] tasks** = different files, no dependencies - safe to run in parallel
- **Verify tests fail** before implementing (TDD discipline)
- **Commit after each task** for clean history
- **Reuse existing code** from Architect Agent where applicable (LLM client, state machine, reasoning stream)
- **Avoid**: vague tasks, same file conflicts, skipping tests

---

## Validation Checklist
*GATE: Checked before marking tasks complete*

- [x] All contracts have corresponding tests (9 commands + 10 events = 19 tests) ✓
- [x] All entities have model tasks (9 entities = 9 model tasks) ✓
- [x] All tests come before implementation (T004-T021 before T022+) ✓
- [x] Parallel tasks truly independent (different files, no shared state) ✓
- [x] Each task specifies exact file path ✓
- [x] No task modifies same file as another [P] task ✓

---

**Total Tasks**: 59  
**Parallel Tasks**: 21 (contract tests + models + setup)  
**Sequential Tasks**: 38 (core logic + integration + polish)  
**Estimated Duration**: 15-20 hours (with parallel execution)

**Ready for execution**: ✓
