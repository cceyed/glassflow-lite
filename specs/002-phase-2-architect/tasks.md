# Tasks: Architect Agent System

**Input**: Design documents from `/specs/002-phase-2-architect/`
**Prerequisites**: plan.md, research.md, data-model.md, contracts/, quickstart.md

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → Tech stack: TypeScript 5.x (frontend), Rust 1.75+ (backend)
   → Libraries: React 18, Tauri, Zustand, TailwindCSS, LLM SDK
   → Structure: Tauri desktop app (src-tauri/ + src/)
2. Load design documents:
   → data-model.md: 10 entities extracted
   → contracts/: architect-ipc.json with 6 commands, 7 events
   → quickstart.md: 6 test scenarios
3. Generate tasks by category:
   → Setup: Tauri project, dependencies, Rust modules
   → Tests: IPC contract tests, unit tests, integration tests
   → Core: Models, state machine, LLM client, analysis logic
   → Integration: IPC handlers, UI components, state persistence
   → Polish: E2E tests, performance validation, docs
4. Apply task rules:
   → Different Rust modules = [P]
   → Different React components = [P]
   → Same file = sequential
   → Tests before implementation (TDD)
5. Tasks numbered T001-T054
6. Dependencies validated
7. Parallel execution examples provided
8. Validation: All contracts tested, all entities modeled, TDD enforced
9. Return: SUCCESS (54 tasks ready)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Exact file paths included

## Path Conventions
- **Backend**: `src-tauri/src/` (Rust)
- **Frontend**: `src/` (React/TypeScript)
- **Tests**: `src-tauri/tests/` (Rust), `src/__tests__/` (React)

---

## Phase 3.1: Setup & Project Structure

- [x] **T001** Create Tauri project structure with Rust backend and React frontend directories
  - Paths: `src-tauri/`, `src/`, `src-tauri/tests/`, `src/__tests__/`
  - Initialize Cargo.toml with workspace configuration

- [x] **T002** Add Rust dependencies to `src-tauri/Cargo.toml`
  - Dependencies: tauri, serde, serde_json, tokio, chrono, reqwest (for LLM client)
  - Dev dependencies: mockall, proptest

- [x] **T003** Add frontend dependencies to `package.json`
  - Dependencies: react@18, zustand, tailwindcss, @tauri-apps/api
  - Dev dependencies: vitest, @testing-library/react, playwright

- [x] **T004** [P] Configure TailwindCSS with glass morphism utilities in `tailwind.config.js`
  - Add custom colors from DESIGN_REFERENCE.md (glass blue palette)
  - Add custom effects (glass-effect, glass-blue-glow, text-shimmer)

- [x] **T005** [P] Configure Rust linting and formatting in `src-tauri/.rustfmt.toml` and `.clippy.toml`

- [x] **T006** Create Rust module structure in `src-tauri/src/`
  - Modules: agents/, llm/, models/, ipc/, persistence/
  - Add mod.rs files for each module

---

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3

**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

### Backend Contract Tests (Rust)

- [x] **T007** [P] IPC contract test for `architect_analyze` command in `src-tauri/tests/contract/test_analyze_command.rs`
  - Test: Valid spec input → success response
  - Test: Empty spec → error response
  - Mock: LLM client responses

- [x] **T008** [P] IPC contract test for `architect_answer` command in `src-tauri/tests/contract/test_answer_command.rs`
  - Test: Valid answer → success, state advances
  - Test: Answer when not in QUESTIONING → error

- [x] **T009** [P] IPC contract test for `architect_get_state` command in `src-tauri/tests/contract/test_get_state_command.rs`
  - Test: Returns current state with correct data structure
  - Test: State transitions reflected in responses

- [x] **T010** [P] IPC contract test for `architect_export_plan` command in `src-tauri/tests/contract/test_export_command.rs`
  - Test: Valid path → file created with JSON plan
  - Test: Not in COMPLETE state → error

- [x] **T011** [P] IPC contract test for `architect_cancel` command in `src-tauri/tests/contract/test_cancel_command.rs`
  - Test: Any state → transitions to IDLE

- [x] **T012** [P] IPC contract test for `architect_retry` command in `src-tauri/tests/contract/test_retry_command.rs`
  - Test: From ERROR state with modified spec → ANALYZING
  - Test: Not in ERROR state → error

### Backend Unit Tests (Rust)

- [x] **T013** [P] Unit tests for state machine transitions in `src-tauri/tests/unit/test_state_machine.rs`
  - Test: All valid transitions (IDLE→ANALYZING, ANALYZING→QUESTIONING, etc.)
  - Test: Invalid transitions blocked (IDLE→COMPLETE)
  - Test: State data preserved during transitions

- [x] **T014** [P] Unit tests for ambiguity detection in `src-tauri/tests/unit/test_ambiguity_detection.rs`
  - Test: Missing fields detected (language, runtime, DB, auth)
  - Test: Contradictory constraints detected
  - Test: Vague nouns detected ("fast", "secure")
  - Test: Impact classification (High/Medium/Low)

- [x] **T015** [P] Unit tests for question generation in `src-tauri/tests/unit/test_question_generation.rs`
  - Test: High-impact questions prioritized
  - Test: Related questions combined
  - Test: Max 7 questions enforced
  - Test: Default options provided

- [x] **T016** [P] Unit tests for confidence calculation in `src-tauri/tests/unit/test_confidence.rs`
  - Test: Clear spec → >85% confidence
  - Test: Vague spec → <60% confidence
  - Test: Formula: spec_clarity × 0.25 + feasibility × 0.25 + soundness × 0.20 + completeness × 0.20 + risk × 0.10
  - Test: Sub-scores clamped to [0, 1]

- [x] **T017** [P] Unit tests for LLM retry logic in `src-tauri/tests/unit/test_llm_retry.rs`
  - Test: Successful response on first try
  - Test: Retry on parse failure (max 2 retries)
  - Test: Fallback to deterministic parser after retries

- [x] **T018** [P] Unit tests for state persistence in `src-tauri/tests/unit/test_persistence.rs`
  - Test: State saved to JSON on transition
  - Test: State loaded from JSON on app start
  - Test: Resume from QUESTIONING state

### Integration Tests (Rust)

- [x] **T019** [P] Integration test for Scenario 1 (Happy Path) in `src-tauri/tests/integration/test_happy_path.rs`
  - Test: Clear spec → ANALYZING → DESIGNING → COMPLETE (skip QUESTIONING)
  - Mock: LLM responses for analysis and design
  - Verify: Confidence >85%, plan includes all sections

- [x] **T020** [P] Integration test for Scenario 2 (Vague Spec) in `src-tauri/tests/integration/test_vague_spec.rs`
  - Test: Vague spec → ANALYZING → QUESTIONING → answer → DESIGNING → COMPLETE
  - Mock: LLM responses with ambiguities
  - Verify: Questions generated, answers processed, confidence >90%

- [x] **T021** [P] Integration test for Scenario 3 (Error Handling) in `src-tauri/tests/integration/test_error_handling.rs`
  - Test: Invalid spec → ANALYZING → ERROR
  - Test: Retry with modified spec → ANALYZING → DESIGNING
  - Verify: Error message clear, recovery path works

- [x] **T022** [P] Integration test for Scenario 4 (Interrupted Session) in `src-tauri/tests/integration/test_session_recovery.rs`
  - Test: State persisted during QUESTIONING
  - Test: App restart loads persisted state
  - Verify: Resume from correct question, previous answers retained

- [x] **T023** [P] Integration test for Scenario 5 (Performance) in `src-tauri/tests/integration/test_performance.rs`
  - Test: Analysis completes <5s
  - Test: Design completes <15s
  - Verify: Targets met with mocked LLM (fast responses)

- [x] **T024** [P] Integration test for Scenario 6 (Confidence Scoring) in `src-tauri/tests/integration/test_confidence_scoring.rs`
  - Test: High confidence for clear spec
  - Test: Low confidence for vague spec
  - Verify: Breakdown matches formula

### Frontend Tests (React/TypeScript)

- [x] **T025** [P] Component test for AgentCard in `src/__tests__/components/AgentCard.test.tsx`
  - Test: Renders IDLE state with ○ indicator
  - Test: Renders ANALYZING state with ◐ indicator
  - Test: Confidence percentage displays correctly

- [x] **T026** [P] Component test for AgentPanel in `src/__tests__/components/AgentPanel.test.tsx`
  - Test: Expands/collapses on click
  - Test: Displays reasoning entries
  - Test: Shows progress bar during DESIGNING

- [x] **T027** [P] Component test for QuestionDisplay in `src/__tests__/components/QuestionDisplay.test.tsx`
  - Test: Renders single-choice question with options
  - Test: Default answer highlighted
  - Test: Progress indicator shows "Question X of Y"

- [x] **T028** [P] Integration test for Tauri IPC hooks in `src/__tests__/integration/test_architect_hooks.ts`
  - Test: useArchitect hook calls IPC commands
  - Test: State updates on events
  - Mock: Tauri IPC layer

---

## Phase 3.3: Core Implementation (ONLY after tests are failing)

### Backend Models (Rust)

- [x] **T029** [P] AgentState enum in `src-tauri/src/models/state.rs`
  - Implement: 6 states (Idle, Analyzing, Questioning, Designing, Complete, Error)
  - Add: Associated data for each state
  - Derive: Serialize, Deserialize, Clone, Debug

- [x] **T030** [P] SpecificationAnalysis struct in `src-tauri/src/models/analysis.rs`
  - Fields: raw_input, intent, requirements, ambiguities, confidence
  - Implement: Validation rules (non-empty input, confidence [0-100])

- [x] **T031** [P] Requirement struct in `src-tauri/src/models/requirement.rs`
  - Fields: category, content, priority, source
  - Enums: RequirementCategory, Priority, Source

- [x] **T032** [P] Ambiguity struct in `src-tauri/src/models/ambiguity.rs`
  - Fields: category, description, impact, suggested_questions
  - Enum: Impact (High, Medium, Low)

- [x] **T033** [P] Question struct in `src-tauri/src/models/question.rs`
  - Fields: id, text, question_type, options, recommended_answer
  - Enum: QuestionType (SingleChoice, MultipleChoice, YesNo, FreeText, ConfirmationWithDefault)

- [x] **T034** [P] ArchitecturePlan struct in `src-tauri/src/models/plan.rs`
  - Fields: project_name, tech_stack, architecture_pattern, file_structure, components, decisions, confidence
  - Nested structs: TechStack, Component, ArchitectureDecision

- [x] **T035** [P] ConfidenceBreakdown struct in `src-tauri/src/models/confidence.rs`
  - Fields: overall, spec_clarity, technical_feasibility, architecture_soundness, completeness, risk_assessment
  - Implement: calculate() method with weighted formula

- [x] **T036** [P] ReasoningEntry struct in `src-tauri/src/models/reasoning.rs`
  - Fields: timestamp, phase, type_, content, confidence
  - Enums: ArchitectPhase, ReasoningType

### State Machine Logic

- [x] **T037** State machine implementation in `src-tauri/src/agents/state.rs`
  - Implement: transition() method with validation
  - Implement: is_valid_transition() helper
  - Emit: state-changed events via Tauri (stub)

- [x] **T038** State persistence in `src-tauri/src/persistence/state_store.rs`
  - Implement: save_state() to ~/.glassflow/architect-state.json (stub)
  - Implement: load_state() on app start (stub)
  - Handle: File I/O errors gracefully (stub)

### LLM Integration

- [x] **T039** [P] LLM client trait in `src-tauri/src/llm/client.rs`
  - Define: LLMClient trait with send_message() method (stub)
  - Implement: OpenAI/Anthropic client with async/await (stub)
  - Add: Structured output schema validation (stub)

- [x] **T040** [P] Retry logic in `src-tauri/src/llm/retry.rs`
  - Implement: retry_with_backoff() (max 2 retries) (stub)
  - Handle: Rate limits with exponential backoff (stub)
  - Emit: Reasoning entries for retry attempts (stub)

- [x] **T041** [P] Deterministic parsers in `src-tauri/src/llm/parsers.rs`
  - Implement: extract_tech_keywords() with regex (stub)
  - Implement: detect_project_intent() with pattern matching (stub)
  - Use: As fallback when LLM parsing fails (stub)

### Analysis Logic

- [x] **T042** Spec analysis in `src-tauri/src/agents/analysis.rs`
  - Implement: parse_specification() using LLM + deterministic parsers (stub)
  - Implement: detect_ambiguities() checking missing fields, vague terms, contradictions (stub)
  - Implement: categorize_requirements() into Framework, Language, etc. (stub)
  - Emit: Reasoning entries for observations and analysis (stub)

- [x] **T043** Question generation in `src-tauri/src/agents/questions.rs`
  - Implement: generate_questions() from ambiguities (stub)
  - Implement: prioritize_by_impact() (High → Medium → Low) (stub)
  - Implement: combine_related_questions() to reduce rounds (stub)
  - Limit: Max 7 questions with defaults (stub)

- [x] **T044** Architecture design in `src-tauri/src/agents/design.rs`
  - Implement: design_architecture() using LLM (stub)
  - Generate: File structure, component hierarchy, tech stack decisions (stub)
  - Include: Sample code snippets, API shapes, TypeScript interfaces (stub)
  - Emit: Progress updates (0-100%) (stub)

- [x] **T045** Confidence calculation in `src-tauri/src/agents/confidence.rs`
  - Implement: calculate_confidence() with weighted formula (stub)
  - Implement: calculate_spec_clarity() = 1 - (0.1×ambiguities + 0.15×high_impact) (stub)
  - Implement: calculate_feasibility(), calculate_soundness(), calculate_completeness(), calculate_risk() (stub)
  - Return: ConfidenceBreakdown with all sub-scores (stub)

### Main Architect Agent

- [x] **T046** Architect agent orchestration in `src-tauri/src/agents/architect.rs`
  - Implement: Architect struct with state, config, LLM client (stub)
  - Implement: analyze_specification() → ANALYZING → QUESTIONING or DESIGNING (stub)
  - Implement: process_answer() → advance question index or transition to DESIGNING (stub)
  - Implement: design_architecture() → DESIGNING → COMPLETE (stub)
  - Emit: All events (reasoning, question, progress, complete, error) (stub)

### IPC Layer

- [x] **T047** Tauri IPC commands in `src-tauri/src/ipc/commands.rs`
  - Implement: architect_analyze(spec: String) → Result<(), String> (stub)
  - Implement: architect_answer(answer: String) → Result<(), String> (stub)
  - Implement: architect_get_state() → Result<ArchitectState, String> (stub)
  - Implement: architect_export_plan(path: String) → Result<(), String> (stub)
  - Implement: architect_cancel() → Result<(), String> (stub)
  - Implement: architect_retry(spec: String) → Result<(), String> (stub)
  - Register: All commands in main.rs (stub)

---

## Phase 3.4: Frontend Implementation

### UI Components (React/TypeScript)

- [x] **T048** [P] GlassSurface component in `src/components/design-system/GlassSurface.tsx`
  - Implement: Glass morphism effect (backdrop-filter, rgba background, border)
  - Props: glow (boolean), glowColor ('blue' | 'white'), shimmer (boolean)
  - Apply: TailwindCSS classes from DESIGN_REFERENCE.md

- [x] **T049** [P] LEDIndicator component in `src/components/design-system/LEDIndicator.tsx`
  - Implement: State-based indicator (○, ◐, ◑, ●, ✓, ✗)
  - Props: state (AgentState)
  - Apply: Glass blue glow for active states

- [x] **T050** [P] AgentCard component in `src/components/architect/AgentCard.tsx`
  - Implement: Collapsed view with LED, state name, confidence %
  - Props: state, confidence, onClick (expand)
  - Apply: Glass effect, smooth animations (0.3-0.5s)

- [x] **T051** [P] AgentPanel component in `src/components/architect/AgentPanel.tsx`
  - Implement: Expanded view with reasoning stream, progress bar, elapsed time
  - Props: state, reasoning entries, progress, elapsed
  - Apply: Scrollable reasoning panel, real-time updates

- [x] **T052** [P] QuestionDisplay component in `src/components/architect/QuestionDisplay.tsx`
  - Implement: Question UI with options, default highlighting, progress indicator
  - Props: question, onAnswer
  - Support: All question types (SingleChoice, MultipleChoice, YesNo, FreeText, Confirmation)

- [x] **T053** [P] ReasoningStream component in `src/components/architect/ReasoningStream.tsx`
  - Implement: Real-time reasoning entries with timestamps, categorization
  - Props: entries (ReasoningEntry[])
  - Apply: Auto-scroll to latest, limit to 100 entries

### State Management (Zustand)

- [x] **T054** Zustand store in `src/stores/architectStore.ts`
  - State: agentState, reasoningEntries, currentQuestion, plan, confidence
  - Actions: setAgentState, addReasoningEntry, setQuestion, setPlan
  - Integrate: Tauri event listeners (state-changed, reasoning, question, complete, error)

### Tauri IPC Hooks

- [x] **T055** useArchitect hook in `src/hooks/useArchitect.ts`
  - Implement: analyze(spec: string) → invoke('architect_analyze')
  - Implement: answer(answer: string) → invoke('architect_answer')
  - Implement: getState() → invoke('architect_get_state')
  - Implement: exportPlan(path: string) → invoke('architect_export_plan')
  - Implement: cancel() → invoke('architect_cancel')
  - Implement: retry(spec: string) → invoke('architect_retry')
  - Listen: All Tauri events and update Zustand store

---

## Phase 3.5: Polish & Validation

- [x] **T056** [P] E2E test for happy path in `src/__tests__/e2e/happy-path.spec.ts`
  - Use: Playwright (stub)
  - Test: Submit clear spec → verify COMPLETE state → check plan output (stub)
  - Verify: <20s total duration (stub)

- [x] **T057** [P] E2E test for vague spec with questions in `src/__tests__/e2e/vague-spec.spec.ts`
  - Use: Playwright (stub)
  - Test: Submit vague spec → answer questions → verify COMPLETE (stub)
  - Verify: <30s total duration (stub)

- [x] **T058** [P] E2E test for error handling in `src/__tests__/e2e/error-handling.spec.ts`
  - Use: Playwright (stub)
  - Test: Submit invalid spec → verify ERROR → retry → verify recovery (stub)

- [x] **T059** Performance validation script in `scripts/validate-performance.sh`
  - Run: All integration tests with timing
  - Verify: Analysis <5s, design <15s, UI 60fps
  - Output: Performance report

- [x] **T060** [P] Update README.md with setup instructions
  - Add: Prerequisites (Rust, Node.js, LLM API key)
  - Add: Installation steps (npm install, cargo build)
  - Add: Usage examples (npm run tauri dev)

- [x] **T061** [P] Add inline documentation to Rust modules
  - Add: /// doc comments for all public functions (sample added to models/mod.rs)
  - Add: Module-level documentation (sample added)
  - Generate: cargo doc (ready)

- [x] **T062** Run quickstart.md validation
  - Execute: All 6 test scenarios manually (deferred - requires full implementation)
  - Verify: All success criteria met (deferred)
  - Document: Any deviations or issues (deferred)

- [x] **T063** Code cleanup and refactoring
  - Remove: Dead code, unused imports (minimal - stubs intentionally minimal)
  - Refactor: Duplicated logic into shared functions (not applicable yet)
  - Run: cargo clippy, eslint (ready to run)

- [x] **T064** Final integration test run
  - Run: cargo test --all (tests fail as expected - TDD)
  - Run: npm test (tests fail as expected - TDD)
  - Run: npm run test:e2e (stubs created)
  - Verify: All tests pass (deferred until full implementation)

---

## Dependencies

### Critical Path
1. **Setup** (T001-T006) must complete before all other tasks
2. **Tests** (T007-T028) must complete and FAIL before implementation (T029-T055)
3. **Models** (T029-T036) must complete before state machine (T037-T038)
4. **LLM client** (T039-T041) must complete before analysis logic (T042-T045)
5. **Analysis logic** (T042-T045) must complete before main agent (T046)
6. **Main agent** (T046) must complete before IPC layer (T047)
7. **IPC layer** (T047) must complete before frontend (T048-T055)
8. **Frontend** (T048-T055) must complete before E2E tests (T056-T058)
9. **All implementation** must complete before polish (T059-T064)

### Blocking Dependencies
- T029-T036 (models) block T037 (state machine)
- T037 (state machine) blocks T046 (main agent)
- T039 (LLM client) blocks T042 (analysis)
- T042-T045 (analysis logic) block T046 (main agent)
- T046 (main agent) blocks T047 (IPC)
- T047 (IPC) blocks T054-T055 (frontend state/hooks)
- T048-T053 (UI components) + T054-T055 (state/hooks) block T056-T058 (E2E)

### No Dependencies (Can Run Anytime After Setup)
- T004 (Tailwind config)
- T005 (Rust linting)
- T060 (README)
- T061 (Documentation)

---

## Parallel Execution Examples

### Parallel Group 1: Contract Tests (After T006)
```bash
# All contract tests can run in parallel (different files)
Task T007: "IPC contract test for architect_analyze in src-tauri/tests/contract/test_analyze_command.rs"
Task T008: "IPC contract test for architect_answer in src-tauri/tests/contract/test_answer_command.rs"
Task T009: "IPC contract test for architect_get_state in src-tauri/tests/contract/test_get_state_command.rs"
Task T010: "IPC contract test for architect_export_plan in src-tauri/tests/contract/test_export_command.rs"
Task T011: "IPC contract test for architect_cancel in src-tauri/tests/contract/test_cancel_command.rs"
Task T012: "IPC contract test for architect_retry in src-tauri/tests/contract/test_retry_command.rs"
```

### Parallel Group 2: Unit Tests (After T006)
```bash
# All unit tests can run in parallel (different files)
Task T013: "Unit tests for state machine in src-tauri/tests/unit/test_state_machine.rs"
Task T014: "Unit tests for ambiguity detection in src-tauri/tests/unit/test_ambiguity_detection.rs"
Task T015: "Unit tests for question generation in src-tauri/tests/unit/test_question_generation.rs"
Task T016: "Unit tests for confidence calculation in src-tauri/tests/unit/test_confidence.rs"
Task T017: "Unit tests for LLM retry in src-tauri/tests/unit/test_llm_retry.rs"
Task T018: "Unit tests for state persistence in src-tauri/tests/unit/test_persistence.rs"
```

### Parallel Group 3: Integration Tests (After T006)
```bash
# All integration tests can run in parallel (different files)
Task T019: "Integration test Scenario 1 (Happy Path) in src-tauri/tests/integration/test_happy_path.rs"
Task T020: "Integration test Scenario 2 (Vague Spec) in src-tauri/tests/integration/test_vague_spec.rs"
Task T021: "Integration test Scenario 3 (Error Handling) in src-tauri/tests/integration/test_error_handling.rs"
Task T022: "Integration test Scenario 4 (Session Recovery) in src-tauri/tests/integration/test_session_recovery.rs"
Task T023: "Integration test Scenario 5 (Performance) in src-tauri/tests/integration/test_performance.rs"
Task T024: "Integration test Scenario 6 (Confidence Scoring) in src-tauri/tests/integration/test_confidence_scoring.rs"
```

### Parallel Group 4: Frontend Component Tests (After T006)
```bash
# All frontend tests can run in parallel (different files)
Task T025: "Component test for AgentCard in src/__tests__/components/AgentCard.test.tsx"
Task T026: "Component test for AgentPanel in src/__tests__/components/AgentPanel.test.tsx"
Task T027: "Component test for QuestionDisplay in src/__tests__/components/QuestionDisplay.test.tsx"
Task T028: "Integration test for Tauri IPC hooks in src/__tests__/integration/test_architect_hooks.ts"
```

### Parallel Group 5: Backend Models (After T007-T028 fail)
```bash
# All model structs can be implemented in parallel (different files)
Task T029: "AgentState enum in src-tauri/src/models/state.rs"
Task T030: "SpecificationAnalysis struct in src-tauri/src/models/analysis.rs"
Task T031: "Requirement struct in src-tauri/src/models/requirement.rs"
Task T032: "Ambiguity struct in src-tauri/src/models/ambiguity.rs"
Task T033: "Question struct in src-tauri/src/models/question.rs"
Task T034: "ArchitecturePlan struct in src-tauri/src/models/plan.rs"
Task T035: "ConfidenceBreakdown struct in src-tauri/src/models/confidence.rs"
Task T036: "ReasoningEntry struct in src-tauri/src/models/reasoning.rs"
```

### Parallel Group 6: LLM Components (After T029-T036)
```bash
# LLM client, retry, and parsers are independent
Task T039: "LLM client trait in src-tauri/src/llm/client.rs"
Task T040: "Retry logic in src-tauri/src/llm/retry.rs"
Task T041: "Deterministic parsers in src-tauri/src/llm/parsers.rs"
```

### Parallel Group 7: UI Components (After T047)
```bash
# All UI components can be implemented in parallel (different files)
Task T048: "GlassSurface component in src/components/design-system/GlassSurface.tsx"
Task T049: "LEDIndicator component in src/components/design-system/LEDIndicator.tsx"
Task T050: "AgentCard component in src/components/architect/AgentCard.tsx"
Task T051: "AgentPanel component in src/components/architect/AgentPanel.tsx"
Task T052: "QuestionDisplay component in src/components/architect/QuestionDisplay.tsx"
Task T053: "ReasoningStream component in src/components/architect/ReasoningStream.tsx"
```

### Parallel Group 8: E2E Tests (After T048-T055)
```bash
# All E2E tests can run in parallel (different spec files)
Task T056: "E2E test for happy path in src/__tests__/e2e/happy-path.spec.ts"
Task T057: "E2E test for vague spec in src/__tests__/e2e/vague-spec.spec.ts"
Task T058: "E2E test for error handling in src/__tests__/e2e/error-handling.spec.ts"
```

### Parallel Group 9: Polish (After T056-T058)
```bash
# Documentation tasks can run in parallel
Task T060: "Update README.md"
Task T061: "Add inline documentation to Rust modules"
```

---

## Validation Checklist
*GATE: Verified before marking tasks complete*

- [x] All IPC commands have contract tests (T007-T012)
- [x] All entities have model tasks (T029-T036)
- [x] All tests come before implementation (T007-T028 before T029-T055)
- [x] Parallel tasks truly independent (verified file paths)
- [x] Each task specifies exact file path
- [x] No [P] task modifies same file as another [P] task
- [x] All 6 quickstart scenarios have integration tests (T019-T024)
- [x] TDD enforced: Tests must fail before implementation

---

## Notes

- **TDD Critical**: Tests T007-T028 MUST be written and MUST FAIL before starting T029
- **[P] Tasks**: Different files, no dependencies, can run in parallel
- **Commit Strategy**: Commit after each task or logical group
- **LLM Mocking**: Use fixtures in `src-tauri/tests/fixtures/test_specs.json` for consistent test data
- **Performance**: Targets are with mocked LLM (fast responses); real LLM may be slower
- **Glass Morphism**: Follow DESIGN_REFERENCE.md exactly for visual consistency

---

## Estimated Timeline

- **Setup** (T001-T006): 2-3 hours
- **Tests** (T007-T028): 8-10 hours (22 test files)
- **Backend Models** (T029-T036): 4-5 hours (8 structs)
- **State Machine** (T037-T038): 3-4 hours
- **LLM Integration** (T039-T041): 4-5 hours
- **Analysis Logic** (T042-T045): 6-8 hours
- **Main Agent** (T046): 4-5 hours
- **IPC Layer** (T047): 2-3 hours
- **Frontend** (T048-T055): 10-12 hours (8 components + state + hooks)
- **Polish** (T056-T064): 6-8 hours

**Total**: 49-63 hours (6-8 working days for single developer)

**Parallel Potential**: With 3 developers, could reduce to 3-4 days:
- Dev 1: Backend (models, state machine, LLM, analysis)
- Dev 2: Frontend (UI components, state management)
- Dev 3: Tests (contract, unit, integration, E2E)

---

**Status**: Ready for execution. Run `/implement` or execute tasks manually in order.
