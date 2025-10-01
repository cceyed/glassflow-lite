# Research: Architect Agent System

**Feature**: 002-phase-2-architect  
**Date**: 2025-09-30  
**Status**: Complete

## Research Questions

### 1. LLM Integration Strategy

**Decision**: Use async Rust client with OpenAI/Anthropic SDK, structured output schema, retry logic

**Rationale**:
- Rust's async/await provides non-blocking LLM calls
- Structured output (JSON schema validation) ensures parseable responses
- Retry logic (max 2) handles transient failures
- Fallback to deterministic parsers (regex + keyword extraction) for critical fields

**Alternatives Considered**:
- Python backend: Rejected - adds language complexity, Tauri is Rust-native
- Synchronous calls: Rejected - blocks UI, poor UX
- No fallback: Rejected - too brittle for production

**Implementation Notes**:
- Use `serde_json` for schema validation
- Implement `LLMClient` trait for swappable providers
- Cache responses to minimize API calls
- Handle rate limits with exponential backoff

### 2. State Machine Implementation Pattern

**Decision**: Enum-based state machine with explicit transitions in Rust

**Rationale**:
- Rust enums with associated data perfect for state machines
- Compile-time guarantees prevent invalid transitions
- Pattern matching ensures exhaustive state handling
- Easy to test and reason about

**Alternatives Considered**:
- State pattern (OOP): Rejected - more verbose in Rust, less idiomatic
- String-based states: Rejected - no type safety
- External state machine library: Rejected - YAGNI, simple enum sufficient

**Implementation Notes**:
```rust
pub enum ArchitectState {
    Idle,
    Analyzing { spec: String, start_time: Instant },
    Questioning { questions: Vec<Question>, answers: HashMap<String, String>, current_index: usize },
    Designing { analysis: SpecificationAnalysis, progress: f32 },
    Complete { plan: ArchitecturePlan, duration: Duration },
    Error { message: String, recoverable: bool },
}

impl ArchitectState {
    pub fn transition(&mut self, event: StateEvent) -> Result<()> {
        // Validate and execute transition
    }
}
```

### 3. Confidence Calculation Approach

**Decision**: Weighted formula with sub-component calculations

**Rationale**:
- Spec clarity: `1 - (0.1 × ambiguities + 0.15 × high_impact)` - penalizes unresolved issues
- Feasibility: Check stack maturity, dependency compatibility, infrastructure (0-1)
- Soundness: Average of separation/scalability/maintainability (0-1)
- Completeness: Percentage of required plan sections (0-1)
- Risk: `1 - normalized_risk` (0-1)
- Overall: Weighted average (clarity 25%, feasibility 25%, soundness 20%, completeness 20%, risk 10%)

**Alternatives Considered**:
- Simple percentage: Rejected - doesn't capture nuance
- ML-based scoring: Rejected - overkill, not interpretable
- Binary pass/fail: Rejected - loses granularity

**Implementation Notes**:
- Expose breakdown to UI for transparency
- Clamp all scores to [0, 1] before weighting
- Document thresholds (e.g., >85% = high confidence, <60% = low)

### 4. Question Generation Strategy

**Decision**: Priority-sorted with question combining and default options

**Rationale**:
- Sort by impact (High → Medium → Low), then blocking likelihood
- Combine related questions (e.g., "framework + language") to reduce rounds
- Max 7 questions prevents user fatigue
- Offer defaults for quick acceptance
- Include "use default" option for all questions

**Alternatives Considered**:
- Ask all questions: Rejected - poor UX, too many rounds
- No defaults: Rejected - slows down experienced users
- Fixed question order: Rejected - doesn't adapt to spec

**Implementation Notes**:
- Question types: SingleChoice, MultipleChoice, YesNo, FreeText, ConfirmationWithDefault
- Balanced style: short context + clear choice + default
- Track progress: "Question 3 of 7"

### 5. UI Component Architecture (Glass Morphism)

**Decision**: React components with Tailwind + custom glass effects, Zustand for state

**Rationale**:
- Tailwind provides utility classes for rapid development
- Custom glass effects (backdrop-filter, rgba backgrounds) per DESIGN_REFERENCE.md
- Zustand lightweight state management (simpler than Redux)
- Component hierarchy: AgentCard (collapsed) → AgentPanel (expanded) → ReasoningStream

**Alternatives Considered**:
- CSS-in-JS (styled-components): Rejected - Tailwind more performant, easier
- Redux: Rejected - overkill for single-agent state
- Vue/Svelte: Rejected - React is project standard

**Implementation Notes**:
- Glass effect: `background: rgba(255,255,255,0.05); backdrop-filter: blur(10px); border: 1px solid rgba(255,255,255,0.1)`
- Blue glow: `box-shadow: 0 0 20px rgba(0,102,255,0.3), 0 0 40px rgba(0,102,255,0.2)`
- Animations: 0.3-0.5s transitions, 60fps target
- LED indicators: Unicode symbols (○, ◐, ◑, ●, ✓, ✗)

### 6. Reasoning Stream Implementation

**Decision**: Event-based streaming with timestamped entries, displayed in UI panel

**Rationale**:
- Real-time updates mask LLM latency
- Categorized entries (Observation, Analysis, Decision, Question, Conclusion)
- Timestamps enable debugging and performance analysis
- Expandable panel keeps UI clean when not needed

**Alternatives Considered**:
- Batch updates: Rejected - poor UX, no real-time feedback
- Verbose logging: Rejected - clutters UI
- No reasoning display: Rejected - reduces transparency

**Implementation Notes**:
```rust
pub struct ReasoningEntry {
    timestamp: DateTime<Utc>,
    phase: ArchitectPhase,
    type_: ReasoningType,
    content: String,
    confidence: Option<f32>,
}
```
- Emit via Tauri events to frontend
- Frontend buffers and displays in scrollable panel
- Limit to last 100 entries to prevent memory bloat

### 7. Architecture Plan Output Format

**Decision**: Structured JSON with comprehensive sections

**Rationale**:
- JSON enables downstream parsing by Engineer Agent
- Comprehensive sections ensure implementability without clarification
- Sample code snippets (file headers, interfaces) reduce ambiguity
- Decisions + reasoning + alternatives provide context for reviewers

**Alternatives Considered**:
- Markdown only: Rejected - harder to parse programmatically
- Minimal output: Rejected - requires follow-up questions
- Code generation: Rejected - out of scope for Architect

**Implementation Notes**:
Required sections:
- Project name & intent
- Tech stack (with versions)
- Architecture pattern
- File structure (paths + purpose + templates)
- Component list (name, purpose, props/state, interactions)
- State management & routing
- Data model / DB schema
- API contracts (endpoints, shapes)
- Build & deployment config
- Testing strategy
- Decisions + reasoning + alternatives
- Confidence breakdown & risks

### 8. Error Handling & Recovery

**Decision**: Explicit error states with recovery paths

**Rationale**:
- Impossible specs (contradictory requirements) → ERROR with resolution suggestions
- LLM failures → retry (max 2) → fallback to deterministic parsers → ERROR if still failing
- User contradictions → ERROR with conflict explanation
- All errors indicate recoverability (can user fix?)

**Alternatives Considered**:
- Silent failures: Rejected - poor UX
- Crash on error: Rejected - unacceptable for desktop app
- Infinite retries: Rejected - wastes API calls

**Implementation Notes**:
- Error state includes: message, recoverable flag, suggested actions
- Transitions: ERROR → IDLE (cancel) or ERROR → ANALYZING (retry with modified spec)
- Log all errors for debugging

### 9. Testing Strategy

**Decision**: TDD with unit, integration, and property tests

**Rationale**:
- Unit tests: State transitions, ambiguity detection, confidence calc, question generation
- Integration tests: Full pipeline (analyze → question → answer → design) with mocked LLM
- Property tests: Random spec generator to verify robustness
- Contract tests: Validate output JSON schema

**Alternatives Considered**:
- Manual testing only: Rejected - not scalable, error-prone
- E2E only: Rejected - slow, doesn't catch unit-level bugs
- No property tests: Rejected - misses edge cases

**Implementation Notes**:
- Use `cargo test` for Rust backend
- Use Vitest for React frontend
- Mock LLM responses with fixtures
- Generate random specs with `proptest` or `quickcheck`

### 10. Performance Optimization

**Decision**: Async LLM calls, streaming UI updates, local state caching

**Rationale**:
- Async prevents blocking (target: analysis <5s, design <15s)
- Streaming reasoning updates provide feedback during long operations
- Local state caching (JSON files) enables resume after interruption
- Graceful degradation if LLM unavailable

**Alternatives Considered**:
- Synchronous: Rejected - blocks UI
- No caching: Rejected - poor UX if interrupted
- Remote state: Rejected - adds complexity, not needed for single-user app

**Implementation Notes**:
- Use Tokio for async runtime
- Stream progress via Tauri events (0-100%)
- Cache state to `~/.glassflow/architect-state.json`
- Display elapsed time in UI

---

## Research Summary

All technical decisions resolved. No NEEDS CLARIFICATION remain. Ready for Phase 1 (Design & Contracts).

**Key Technologies**:
- Backend: Rust 1.75+, Tauri, async/await, serde_json
- Frontend: React 18, TypeScript 5.x, Tailwind CSS, Zustand
- LLM: OpenAI/Anthropic SDK with retry + fallback
- Testing: cargo test, Vitest, Playwright, property tests

**Architecture Pattern**: State machine (6 states) with event-driven transitions

**Risk Mitigation**:
- LLM failures: Retry + deterministic fallback
- Rate limits: Exponential backoff
- Invalid specs: Explicit error states with recovery
- Performance: Async + streaming + caching
